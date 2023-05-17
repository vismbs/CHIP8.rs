use crate::ram::Ram;
pub struct CHIP8 { 
    ram: Ram,  
}

impl CHIP8 {
    pub fn new() -> CHIP8 {
        CHIP8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>){
        let offset = 0x200;
        for i in 0..rom.len() {
            self.ram.write_bytes((offset + i) as u16, rom[i])
        }
    }
}