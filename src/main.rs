use std::fs::File;
use std::io::Read;

mod ram;
mod chip;
fn main() {
   let mut file = File::open("roms/INVADERS").unwrap();
   let mut data: Vec<u8> = Vec::new();
   file.read_to_end(&mut data).expect("Unable to Read the ROM");
   let mut chip8 = chip::CHIP8::new();
   chip8.load_rom(&data);
}
