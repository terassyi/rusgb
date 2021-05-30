use super::mem::ram::Ram;
use super::mem::rom::Rom;

#[derive(Debug)]
pub struct Cartridge<'a> {
    rom: &'a Rom,
    ram: &'a mut Ram,
    mbc_type: bool,
    mode: bool,
}

impl<'a> Cartridge<'a> {
    pub fn new(rom: &'a Rom, ram: &'a mut Ram, mbc_type: bool, mode: bool) -> Cartridge<'a> {
        Cartridge {
            rom: rom,
            ram: ram,
            mbc_type: mbc_type,
            mode: mode,
        }
    }
}
