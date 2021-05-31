use crate::error::*;

pub trait Device {
    fn read(&self, addr: u16) -> GBResult<u8>;
    fn write(&mut self, addr: u16, val: u8) -> GBResult<()>;
}
