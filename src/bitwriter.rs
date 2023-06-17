// simple bit writer
pub struct BitWriter {
    buffer : Vec<u8>,
    _8_bit_buffer : Vec<u8>,
    current_index : usize,
    current_bit : usize
}

pub trait Functions {
    fn flush(&mut self);
    fn append_bit(&mut self, bit : u8);
    fn return_buffer(&self) -> &[u8];
    fn append_byte_alinged(&mut self, byte : u8);
    fn append_byte_unalinged(&mut self, byte : u8);
    fn print_buffer(&self);
}

impl Functions for BitWriter {

    fn flush(&mut self){
        // forcefully flush buffer
        let mut byte :u8 = 0;

        for i in 0 .. 8{

            byte += self._8_bit_buffer[7 - i] << i;
            self._8_bit_buffer[7 - i] = 0;

        }

        self.buffer[self.current_index] = byte;
        self.current_bit = 0;
        self.current_index +=1;
    }

    fn append_bit(&mut self, bit : u8) { 

        if self.current_index == self.buffer.len() as usize{
            self.buffer.resize(self.buffer.len() + 8196, 0)
        }   
        
        if self.current_bit > 7{
            // reset buffer

            let mut byte: u8 = 0;


            for i in 0..8{

                byte += self._8_bit_buffer[7-i] << i;

                self._8_bit_buffer[7-i] = 0;
            }

            self.buffer[self.current_index] = byte;

            self.current_bit = 0;
            self.current_index +=1;
        }

        self._8_bit_buffer[self.current_bit] = bit;

        self.current_bit +=1;
    }
    
    fn append_byte_unalinged(&mut self, byte : u8){

        for i in 0..8{
            self.append_bit((byte>>7-i)&1);
        }
      

    }
    fn append_byte_alinged(&mut self, byte : u8){
        self.buffer[self.current_index] = byte;
        self.current_index +=1;
    }

    fn print_buffer(&self){
        for i in 0..8{
            print!(" {} ",self._8_bit_buffer[i]);
        }
    }

    fn return_buffer(&self) -> &[u8]{
        return &self.buffer[.. (self.current_index)];
    }
}

pub fn new() -> BitWriter{
    
    return BitWriter{
        buffer : vec![0;8196],
        _8_bit_buffer : vec![0;8],
        current_index : 0,
        current_bit :0
    };
    
}