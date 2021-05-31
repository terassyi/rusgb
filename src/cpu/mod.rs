mod register;
mod bus;
mod instruction;

use register::*;
use bus::*;
use instruction::*;
use crate::error::*;

pub struct Cpu<'a> {
    register: &'a mut Register,
    bus: &'a mut Bus<'a>,
    cycle: usize,
    debug: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(reg: &'a mut Register, bus: &'a mut Bus<'a>, debug: bool) -> Cpu<'a> {
        Cpu {
            register: reg,
            bus: bus,
            cycle: 0usize,
            debug: debug,
        }
    }

    pub fn step(&self) -> GBResult<()> {
        let inst = self.fetch()?;
        let f = self.decode(inst)?;
        let consumed_cycle = self.exec(f);
        Ok(())
    }

    fn fetch(&self) -> GBResult<u8> {
        Ok(0u8)
    }

    fn decode(&self, inst: u8) -> GBResult<InstructionFn> {
        Ok(dummy)
    }
    
    fn exec(&self, f: InstructionFn) -> GBResult<usize> {
        Ok(0usize)
    }
}

enum Ime {
    
}
