use crate::cartridge::Cartridge;
use crate::mem::ram::Ram;
use crate::timer::Timer;

#[derive(Debug)]
pub struct Bus<'a> {
    cartridge: &'a mut Cartridge<'a>,
    ram: &'a mut Ram,
    hram: &'a mut Ram,
    timer: &'a mut Timer,
}

impl<'a> Bus<'a> {
    pub fn new(ram: &'a mut Ram, hram: &'a mut Ram, cart: &'a mut Cartridge<'a>, timer: &'a mut Timer) -> Bus<'a> {
        Bus {
            cartridge: cart,
            ram: ram,
            hram: hram,
            timer: timer,
        }
    }
}
