use crate::bitwriter::Functions;
use std::{fs::File, io::Write};
use clap::Parser;

mod lib;
mod bitwriter;

// X,Y to index
fn return_index_from_xy(width : u16, x : u16, y : u16) -> usize{
    
    let width = width as usize;
    let x  = x as usize;
    let y  = y as usize;

    return width * y + x
}

// Main compression
fn recursive_block_compression(
    
    yuv_array : &Vec<u8>,

    mut block : u8,

    min_max_dist : u8,

    depth_compression_level : u8,

    last_pixel : &mut i16,
    last_pixel_color_limit_count : &mut u8,

    depth : u8,

    // slice coords
    x1 : u16 ,
    y1 : u16 ,
    x2 : u16 ,
    y2 : u16 ,
    // convert 2d indexs to 1d
    width : u16,
    writer : &mut bitwriter::BitWriter
    ){
   
    // get min, max of the pixel block
    let mut min : u8 = yuv_array[return_index_from_xy(width,x1,y1)];

    let mut max : u8 = min;

    for y in y1..y2 {

        for x in x1..x2 {

            let pixel : u8 = yuv_array[return_index_from_xy(width, x, y)];

            if pixel < min {
                min = pixel;
            }

            if pixel > max {
                max = pixel;
            }

        }
        
    }

    // Distance between the minimum and maximum pixel
    {
        let dist: f32 = (max-min) as f32;
        let min_max_dist : f32 = min_max_dist as f32;
        let depth = depth as f32;
        let depth_compression_level = depth_compression_level as f32;
    
        // Is the distance is within the allowed threshold
        if block == 1 || dist <= ((min_max_dist) * (depth+1.0)*depth_compression_level)/block as f32 {
            
            // is a leaf, so we flag it with one
            writer.append_bit(1);

            // we'd compute the averages

            let mut avr: f32 = 0.0;
            let mut average_pixel : i16 = 0;

            let block : u32 = block as u32;

            for y in y1..y2 {

                for x in x1..x2{

                    let pixel = yuv_array[return_index_from_xy(width,x,y)];
                    avr += pixel as f32;

                }
            }
            
            avr /= (block*block) as f32;
            average_pixel = avr as i16;

            // now now we decide 
            // 1 is for "yes! the colour is the same!"
            // 0 is for a different colour, followed by a value encoded as an unalinged byte
            let last_pixel_distance = (*last_pixel as f32 - avr).abs();
            
            // for some images, this may be effective 
            // we can save around ~100kb with this 
            // or with we're lucky, ~300
            if last_pixel_distance <= 4.0 && *last_pixel_color_limit_count <4{

                writer.append_bit(1);
                *last_pixel_color_limit_count += 1;
                return;

            }
            
            // different colour
            writer.append_bit(0);

            writer.append_byte_unalinged(average_pixel as u8);
            
            *last_pixel_color_limit_count = 0;
            *last_pixel = average_pixel;


            return;
        }

    }  

  
    // is a node, so we flag it with zero
    
    block /=2;
    writer.append_bit(0);

    for y in ( y1 .. y2 ).step_by(block as usize) {

        for x in ( x1 .. x2 ).step_by(block as usize){

            recursive_block_compression(
                yuv_array, 
                
                block, 

                min_max_dist, 
                depth_compression_level, 

                last_pixel, 
                last_pixel_color_limit_count,

                depth+1,

                x, 
                y, 
                x+block as u16, 
                y+block as u16, 

                width,
                writer
            )

        }

    }


}

fn compress(
    
    yuv_array : &Vec<u8> ,

    width : u16,
    height : u16, 

    min_max_dist : u8,
    depth_compression_level : u8,

    block : u8,
    writer : &mut bitwriter::BitWriter){
   
    for y in (0..height).step_by(block as usize){
        
        for x in (0..width).step_by(block as usize){
            
            let mut last_pixel: i16 = 255;
            let mut last_pixel_color_limit_count: u8 = 0;

            recursive_block_compression(

                yuv_array,

                block,

                min_max_dist,
                depth_compression_level,

                &mut last_pixel,
                &mut last_pixel_color_limit_count,
                0,

                x, 
                y,
                x+block as u16, 
                y+block as u16, 
                width,
                writer
            );
            
        }
    }
    
}


// Arguments


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arugments{

    /// image file
    filename : String,
    /// luminance (y) pixel block size (must be within the powers of 2, max is 128) a pixel size of 8 is 8x8
    lum_block : u8, 
    /// chrominance (u,v) pixel block size (must be within the powers of 2, max is 128) a pixel size of 8 is 8x8
    color_block : u8,
    /// luminance (y) allowed distance
    lum_dist : u8,
    /// chrominance (u,v) allowed distance
    color_dist : u8,
    /// luminance (y) depth compression intensity
    depth_compression_lum : u8,
    /// chrominance (u,v) depth compression intensity
    depth_compression_color : u8,
    /// resize image by width x height, optional 
    #[arg(short,long,num_args(2))] 
    resize : Option<Vec<u16>>,
}


fn main(){

    
    let args = Arugments::parse();

    let filename: &str =  &args.filename;
    
    // Block Size
    let lum_block: u8 = args.lum_block;
    let color_block: u8 = args.color_block;

    // Distance 
    let lum_dist: u8 = args.lum_dist;
    let color_dist: u8 = args.color_dist;

    // The futher the depth, the less detail
    let depth_compression_lum : u8 = args.depth_compression_lum;
    let depth_compression_color : u8 = args.depth_compression_color;

    // currently Returns the resolution
    let res: (u16, u16) = lib::ffprobe(filename);

    let mut _x : u16 = 128*(res.0/128);
    let mut _y : u16 = 128*(res.1/128);

    if args.resize.is_some() {
        
        let resize = args.resize.unwrap();
        
        _x = 128 * (resize[0]/128);
        _y = 128 * (resize[1]/128);

       
    }
    
    let (y,u,v) = lib::ffmpeg_read_image(
        filename, 
        _x, 
        _y, 
    );

    // Do compression on each of the y,u,v

    let mut bitarray = bitwriter::new();
    
    compress(
        &y, 
        _x, 
        _y, 
        lum_dist, 
        depth_compression_lum, 
        lum_block,
        &mut bitarray
    );
 
    compress(
        &u, 
        _x/2, 
        _y/2, 
        color_dist, 
        depth_compression_color, 
        color_block,
        &mut bitarray
    );

    compress(
        &v, 
        _x/2, 
        _y/2, 
        color_dist, 
        depth_compression_color, 
        color_block,
        &mut bitarray
    );

    bitarray.flush();
    let array = bitarray.return_buffer();
    // output to file
    let mut file = File::create("output").unwrap();

    println!("done");

    //amongus
    file.write_all(b"QGIF");
    file.write_all(&lum_block.to_le_bytes());
    file.write_all(&color_block.to_le_bytes());
    file.write_all(&_x.to_be_bytes());
    file.write_all(&_y.to_be_bytes());
    file.write_all(array);
    
  
}