use std::{fs::File, io::Read};


pub struct BitReader {
    
    stream : Vec<u8>,
    cursor : usize,
    cursor_bytes : usize,
    should_return_as_little_endian : bool


}

pub trait Functions {

    fn read_bytes_alinged(&mut self, bytes : usize) -> u32;
    fn read_bits_unalinged(&mut self, bits : usize) -> u32;
    fn return_as_little_endian(&mut self,bool : bool);
    fn read_next_bit(&mut self) -> u8;

}



impl Functions for BitReader { 

    fn return_as_little_endian(&mut self,bool : bool) {
        self.should_return_as_little_endian = bool;
    }

    fn read_bits_unalinged(&mut self, bits : usize) -> u32{

        let mut byte : u32 = 0;

        for i in 0..bits{

            byte += (self.read_next_bit() as u32) <<  ( ( bits - 1) - i) as u32;

        }


       
       return byte;
    }

    fn read_bytes_alinged(&mut self,bytes : usize) -> u32{
        let mut byte : u32 = 0;

        for i in 0..bytes{
            
            byte = byte << 8 | self.stream[self.cursor_bytes] as u32;
            self.cursor_bytes+=1;
        }


        return byte;
    }
    fn read_next_bit(&mut self) -> u8{

        if self.cursor > 7{
           self.cursor_bytes +=1;
           self.cursor = 0;
        }
        
        let bit = self.stream[self.cursor_bytes] >> 7-self.cursor & 1;

        self.cursor +=1;

        return bit; 
    }


}
pub fn new(filename : &str) -> BitReader{
    
    let mut file = File::open(filename).unwrap();
    let mut buffer = vec![];
    
    file.read_to_end(&mut buffer).unwrap();

    return BitReader{
        stream : buffer,
        cursor : 0,
        cursor_bytes : 0,
        should_return_as_little_endian : false
    }
}