use std::io::Read;
use std::process::Command;
use std::process::Stdio;

pub fn ffprobe(filename : &str ) -> (u16,u16){
        
        // I might add some extra stuff here, i'm not sure when or if i care at all

        let ffmpeg_yuv = Command::new("ffprobe")

            .args([filename,
                    "-print_format",
                    "json",
                    "-show_format",
                    "-show_streams",
                    "-v",
                    "quiet"])        
   
            .output()
            .expect("can not execute ffprobe");

        let ffmpeg_output = String::from_utf8(ffmpeg_yuv.stdout).unwrap();      

        let ffprobe_result = json::parse(&ffmpeg_output).unwrap();
        let ffprobe_streams = &ffprobe_result["streams"][0];

        let resolution = (
            ffprobe_streams["width"].as_u16().unwrap(),
            ffprobe_streams["height"].as_u16().unwrap()
        ); 

        return resolution;
}

pub fn ffmpeg_read_image(filename : &str, resx : u16, resy : u16 ) -> (Vec<u8>,Vec<u8>,Vec<u8>){

    // open ffmpeg and read stdout

    let mut ffmpeg = Command::new("ffmpeg") 
        .args(
            [
                "-loglevel","error",
                "-i",filename,
                "-vcodec","rawvideo",
                "-s",&format!("{}x{}",resx,resy),
                "-f","image2pipe",
                "-pix_fmt","yuv420p",
                "-"
            ]
        ) 
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let resx : u32 = resx as u32;
    let resy : u32 = resy as u32;

    let total_y_resolution: usize = (resx * resy) as usize;
    let total_u_v_resolution: usize = ( (resx / 2) * (resy / 2)  ) as usize;
    
    let mut y : Vec<u8> = vec![0;total_y_resolution];
    let mut u : Vec<u8> = vec![0;total_u_v_resolution];
    let mut v : Vec<u8> = vec![0;total_u_v_resolution];

    let mut stdout = ffmpeg.stdout.take().unwrap();

    stdout.read_exact(&mut y).unwrap();
    
    stdout.read_exact(&mut u).unwrap();

    stdout.read_exact(&mut v).unwrap();
    
    ffmpeg.kill().unwrap();

    return (y,u,v)



}