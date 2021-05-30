use crate::error::*;
use crate::util;


pub const REG_B: usize = 0;
pub const REG_C: usize = 1;
pub const REG_D: usize = 2;
pub const REG_E: usize = 3;
pub const REG_H: usize = 4;
pub const REG_L: usize = 5;
pub const REG_HL_R8: usize = 6;
pub const REG_A: usize = 7;

pub const REG_BC: usize = 0;
pub const REG_DE: usize = 1;
pub const REG_HL: usize = 2;
pub const REG_SP: usize = 3;

#[derive(Debug)]
pub struct Register {
    a: u8, // accumlator
    f: u8, // flag
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16, // stack pointer
    pc: u16, // program counter
}

impl Register {
    pub fn new() -> Register {
        Register {
            a: 0u8,
            f: 0u8,
            b: 0u8,
            c: 0u8,
            d: 0u8,
            e: 0u8,
            h: 0u8,
            l: 0u8,
            sp: 0u16,
            pc: 0u16
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn f(&self) -> u8 {
        self.f
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn e(&self) -> u8 {
        self.e
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn af(&self) -> u16 {
        util::u8_to_u16(self.a, self.f)
    }

    pub fn bc(&self) -> u16 {
        util::u8_to_u16(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        util::u8_to_u16(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        util::u8_to_u16(self.h, self.l)
    }

    pub fn sp(&self) -> u16 {
        self.sp
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn set_f(&mut self, val: u8) {
        self.f = val;
    }

    pub fn set_b(&mut self, val: u8) {
        self.b = val;
    }

    pub fn set_c(&mut self, val: u8) {
        self.c = val;
    }

    pub fn set_d(&mut self, val: u8) {
        self.d = val;
    }

    pub fn set_e(&mut self, val: u8) {
        self.e = val;
    }

    pub fn set_h(&mut self, val: u8) {
        self.h = val;
    }

    pub fn set_l(&mut self, val: u8) {
        self.l = val;
    }

    pub fn set_af(&mut self, val: u16) {
        let (a, f) = util::split_u16(val);
        self.a = a;
        self.f = f;
    }

    pub fn set_bc(&mut self, val: u16) {
        let (b, c) = util::split_u16(val);
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, val: u16) {
        let (d, e) = util::split_u16(val);
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, val: u16) {
        let (h, l) = util::split_u16(val);
        self.h = h;
        self.l = l;
    }
    
    pub fn set_sp(&mut self, val: u16) {
        self.sp = val
    }
    
    pub fn set_pc(&mut self, val: u16) {
        self.pc = val
    }

    pub fn get_r8(&self, index: usize) -> GBResult<u8> {
        match index {
            REG_B => Ok(self.b()),
            REG_C => Ok(self.c()),
            REG_D => Ok(self.d()),
            REG_E => Ok(self.e()),
            REG_H => Ok(self.h()),
            REG_L => Ok(self.l()),
            REG_A => Ok(self.a()),
            REG_HL_R8 => Ok(0u8), // read from memory addressed by HL
            _ => Err(GBError::InvalidInput),
        }
    }

    pub fn get_r16(&self, index: usize) -> GBResult<u16> {
        match index {
            REG_BC => Ok(self.bc()),
            REG_DE => Ok(self.de()),
            REG_HL => Ok(self.hl()),
            REG_SP => Ok(self.sp()),
            _ => Err(GBError::InvalidInput),
        }
    }

    pub fn set_r8(&mut self, index: usize, val: u8) -> GBResult<()> {
        match index {
            REG_B => self.set_b(val),
            _ => return Err(GBError::InvalidInput),
        }
        Ok(())
    }
}
