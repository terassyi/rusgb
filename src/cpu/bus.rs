use crate::cartridge::Cartridge;
use crate::mem::ram::Ram;

#[derive(Debug)]
pub struct Bus<'a> {
    cartridge: &'a mut Cartridge<'a>,
    ram: &'a mut Ram,
    hram: &'a mut Ram,
}

impl<'a> Bus<'a> {
    pub fn new(ram: &'a mut Ram, hram: &'a mut Ram, cart: &'a mut Cartridge<'a>) -> Bus<'a> {
        Bus {
            cartridge: cart,
            ram: ram,
            hram: hram,
        }
    }
}
