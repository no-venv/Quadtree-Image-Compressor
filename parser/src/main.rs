use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use clap::Parser;
use crate::bitreader::Functions;
mod bitreader;

fn return_index_from_xy(width : u16, x : u16, y : u16) -> usize{
    
    let width : u32 = width as u32;
    let x : u32 = x as u32;
    let y : u32 = y as u32;

    return (width * y + x) as usize
}

fn fill_colour(
    yuv_array : &mut Vec<u8>,
    value : u8,
    width : u16,
    x : u16,
    y : u16,
    x2 : u16,
    y2 : u16){
    
    for y in y..y2{
        for x in x..x2{
            yuv_array[return_index_from_xy(width, x, y)] = value;
        }
    }
}

fn decode_recursive(
    yuv_array : &mut Vec<u8>,

    last_pixel : &mut u8,

    mut block : u8,

    x : u16,
    y : u16,
    x2 : u16,
    y2 : u16,

    width : u16,
    reader : &mut bitreader::BitReader

){
    // determine if leaf or node

    // 1 = leaf
    // 0 = node

    let the_moment_of_truth = reader.read_next_bit();
    // is a leaf
    if the_moment_of_truth == 1{
        // read to see if the last pixel should be used
        
        let use_last_pixel = reader.read_next_bit();

        if use_last_pixel ==1{
            fill_colour(yuv_array, *last_pixel,width,x,y,x2,y2);
            return;
        }

        let colour = reader.read_bits_unalinged(8) as u8;
        fill_colour(yuv_array,colour,width,x,y,x2,y2);

        *last_pixel = colour;
        
        return;

    }
    // node


    block /=2;

    for y in (y..y2).step_by(block as usize){

        for x in (x..x2).step_by(block as usize){

            decode_recursive(
                yuv_array, 
                last_pixel, 
                block, 
                x, 
                y, 
                x+block as u16, 
                y+block as u16, 
                width, 
                reader
            )

        }
    }


}
fn decode(
    yuv_array : &mut Vec<u8>,
    
    width : u16,
    height : u16,

    block : u8,

    reader : &mut bitreader::BitReader){
    
    let mut last_pixel : u8 = 255;

    for y in (0..height).step_by(block as usize){

        for x in (0..width).step_by(block as usize){

            decode_recursive(
                yuv_array, 
                &mut last_pixel, 
                block,
                x, 
                y, 
                x+block as u16, 
                y+block as u16, 
                width, 
                reader
            )
        }
    }
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Arugments{
    /// image file
    filename : String,
}

fn main() {
    
    let arg = Arugments::parse();
    let header = b"QGIF";
    
    let mut reader = bitreader::new(&arg.filename);
    reader.return_as_little_endian(false);


    // so we read the header

    for i in 0..4{
        if header[i] != reader.read_bytes_alinged(1) as u8 {
            // 💀
            panic!("expected header to be QGIF");
        }
    }


    // then the stuff we need to know

    let lum_block = reader.read_bytes_alinged(1) as u8;
    let color_block = reader.read_bytes_alinged(1) as u8;

    let x = reader.read_bytes_alinged(2);
    let y = reader.read_bytes_alinged(2);

    let total_y = (x*y) as usize;
    let total_uv = (x/2 * y/2) as usize;

    let mut y_decoded_data = vec![0u8;total_y];
    let mut u_decoded_data = vec![0u8;total_uv];
    let mut v_decoded_data = vec![0u8;total_uv];

    println!("lum block : {}, color block : {}, x : {}, y : {}",lum_block,color_block,x,y);
    
    let x = x as u16;
    let y = y as u16;
    
    // then we decodd
    decode(
        &mut y_decoded_data,
        x,
        y, 
        lum_block, 
        &mut reader
    );

    decode(
        &mut u_decoded_data,
        x/2,
        y/2, 
        color_block, 
        &mut reader
    );

    decode(
        &mut v_decoded_data,
        x/2,
        y/2, 
        color_block, 
        &mut reader
    );

    let mut yuv_concat: Vec<u8> = vec![];

    yuv_concat.extend(&y_decoded_data);
    yuv_concat.extend(&u_decoded_data);
    yuv_concat.extend(&v_decoded_data);

    // conversion by ffmpeg
    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-loglevel","error",
            "-f","rawvideo",
            "-pix_fmt", "yuv420p",
            "-s",&format!("{}x{}",x,y),
            "-i","pipe:0",

            "-f","image2",
            "-pix_fmt","rgb24",
            "-y",
            "output.png"

        ]
    )
    .stdin(Stdio::piped())
    .spawn()
    .unwrap();

    let mut stdin = ffmpeg.stdin.take().unwrap();
    stdin.write_all(&yuv_concat);

}
