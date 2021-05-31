use crate::error::*;
use super::register::Register;
use super::bus::Bus;

// instruction operation fn(instruction opcode, register, bus) -> consumed clock cycle
pub type InstructionFn = fn(inst: u8, reg: &mut Register, bus: &mut Bus) -> GBResult<usize>;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    // 8bit load 
    LD_R_R, // xx
    LD_R_N, // xx nn
    LD_R_HL, // xx
    LD_HL_R, // 7x
    LD_HL_N, // 36 nn
    LD_A_BC, // 0x0a
    LD_A_DE, //0x1a
    LD_A_NN, // 0xfa
    LD_BC_A, // 0x02
    LD_DE_A, // 0x12
    LD_NN_A, // 0xea
    LD_A_IO_N, // 0xf0 nn
    LD_IO_N_A, // 0xe0 nn
    LD_A_IO_C, // 0xf2
    LD_IO_C_A, // 0xe2
    LDI_HL_A, // 0x22
    LDI_A_HL, // 0x2a
    LDD_HL_A, // 0x32
    LDD_A_HL, //0x3a
    // 16bit load
    LD_RR_NN, // 0xx1 nn nn
    LD_SP_HL, // 0xf9
    LD_NN_SP, // 0x08
    PUSH_RR, // 0xx5
    POP_RR, // 0xx1
    // 8bit arithmethic/logic
    ADD_A_R, // 0x8x
    ADD_A_N, // 0xc6 nn
    ADD_A_HL, // 0x86
    ADC_A_R, // 0x8x
    ADC_A_N, // 0xce nn
    ADC_A_HL, // 0x8e
    SUB_R, // 0x9x
    SUB_N, // 0xd6 nn
    SUB_HL, // 0x96
    SBC_A_R, // 0x9x
    SBC_A_N, // 0xde nn
    SBC_A_HL, // 0x9e
    AND_R, // 0xax
    AND_N, // 0xe6 nn
    AND_HL, // 0xax
    XOR_R, // 0xax
    XOR_N, // 0xee nn
    XOR_HL, // 0xae
    OR_R, // 0xbx
    OR_N, // 0xf6 nn
    OR_HL, // 0xb6
    CP_R, // 0xbx
    CP_N, // 0xfe nn
    CP_HL, // 0xbe
    INC_R, // 0xxx
    INC_HL, // 0x34
    DEC_R, // 0xxx
    DEC_HL, // 0x35
    DAA, // 0x27
    CPL, // 0x2f
    // 16bit arithmethic/logic
    ADD_HL_RR, // 0xx9
    INC_RR, // 0xx3
    DEC_RR, // 0xxb
    ADD_SP_DD, // 0xe8
    LD_HL_SP_DD, // 0xf8
    RLCA, // 0x07
    RLA, // 0x17
    RRCA, // 0x0f
    RRA, // 0x1f
    RLC_R, // 0xcb 0x
    RLC_HL, // 0xcb 06
    RL_R, // cb 1x
    RL_HL, // cb 16
    RRC_R, // cb 0x
    RRC_HL, // cb 0e
    RR_R, // cb 1x
    RR_HL, // cb 1e
    SLA_R, // cb 2x
    SLA_HL, // cb 26
    SWAP_R, // cb 3x
    SWAP_HL, // cb 36
    SRA_R, // cb 2x
    SRA_HL, // cb 2e
    SRL_R, // cb 3x
    SRL_HL, // cb 3e
    // single bit operation
    BIT_N_R, // cb xx
    BIT_N_HL, // cb xx
    SET_R, // cb xx
    SET_HL, // cb xx
    RES_R, // cb xx
    RES_HL, // cb xx
    // cpu control
    CCF, // 3f
    SCF, // 37
    NOP, // 0x00
    HALT, // 76
    STOP, // 10 00
    DI, // f3
    EI, // fb
    // jump
    JP_NN, // c3 nn nn
    JP_HL, // e9
    JP_F_NN, // xx nn nn
    JR_PC_DD, // 18 dd
    JR_F_PC_DD, // xx dd
    CALL_NN, // cd nn nn
    CALL_F_NN, // xx nn nn
    RET, // c9
    RET_F, // xx
    RETI, // d9
    RST, // xx
    PREFIX, // cb
}

impl Instruction {
    pub fn from(inst: u8) -> GBResult<Instruction> {
        match inst {
            0x00 => Ok(Instruction::NOP),
            0x76 => Ok(Instruction::HALT),
            // load
            0x01 | 0x11 | 0x21 | 0x31 => Ok(Instruction::LD_RR_NN),
            0x02 => Ok(Instruction::LD_BC_A),
            0x12 => Ok(Instruction::LD_DE_A),
            0x0a => Ok(Instruction::LD_A_BC),
            0x1a => Ok(Instruction::LD_A_DE),
            0x22 => Ok(Instruction::LDI_HL_A),
            0x32 => Ok(Instruction::LDD_HL_A),
            0x2a => Ok(Instruction::LDI_A_HL),
            0x3a => Ok(Instruction::LDD_A_HL),
            0x06 | 0x16 | 0x26 | 0x36 | 0x0e | 0x1e | 0x2e | 0x3e => Ok(Instruction::LD_R_N),
            0x08 => Ok(Instruction::LD_NN_SP),
            0x40..=0x45 | 0x47 | 0x48..=0x4d | 0x4f | 0x50..=0x55 | 0x57 | 0x58..=0x5d | 0x5f |
            0x60..=0x65 | 0x67 | 0x68..=0x6d | 0x6f | 0x78..=0x7d | 0x7f => Ok(Instruction::LD_R_R),
            0x46 | 0x4e | 0x56 | 0x5e | 0x66 | 0x6e | 0x7e => Ok(Instruction::LD_R_HL),
            0x70..=0x75 | 0x77 => Ok(Instruction::LD_HL_R),
            0xe0 => Ok(Instruction::LD_IO_N_A),
            0xf0 => Ok(Instruction::LD_A_IO_N),
            0xe2 => Ok(Instruction::LD_IO_C_A),
            0xf2 => Ok(Instruction::LD_IO_C_A),
            0xf8 => Ok(Instruction::LD_HL_SP_DD),
            0xf9 => Ok(Instruction::LD_SP_HL),
            0xea => Ok(Instruction::LD_NN_A),
            0xfa => Ok(Instruction::LD_A_NN),
            // push/pop
            0xc1 | 0xd1 | 0xe1 | 0xf1 => Ok(Instruction::POP_RR),
            0xc5 | 0xd5 | 0xe5 | 0xf5 => Ok(Instruction::PUSH_RR),
            // arithmethic
            0x09 | 0x19 | 0x29 | 0x39 => Ok(Instruction::ADD_HL_RR),
            0x03 | 0x13 | 0x23 | 0x33 => Ok(Instruction::INC_RR),
            0x0b | 0x1b | 0x2b | 0x3b => Ok(Instruction::DEC_RR),
            0x34 => Ok(Instruction::INC_HL),
            0x04 | 0x14 | 0x24 | 0x0c | 0x1c | 0x2c | 0x3c => Ok(Instruction::INC_R),
            0x05 | 0x15 | 0x25 | 0x0d | 0x1d | 0x2d | 0x3d => Ok(Instruction::DEC_R),
            0x35 => Ok(Instruction::DEC_HL),
            0x80..=0x85 | 0x87 => Ok(Instruction::ADD_A_R),
            0x86 => Ok(Instruction::ADD_A_HL),
            0x88..=0x8d | 0x8f => Ok(Instruction::ADC_A_R),
            0x8e => Ok(Instruction::ADC_A_HL),
            0x90..=0x95 | 0x97 => Ok(Instruction::SUB_R),
            0x96 => Ok(Instruction::SUB_HL),
            0x98..=0x9d | 0x9f => Ok(Instruction::SBC_A_R),
            0x9e => Ok(Instruction::SBC_A_HL),
            0xc6 => Ok(Instruction::ADD_A_N),
            0xce => Ok(Instruction::ADC_A_N),
            0xd6 => Ok(Instruction::SUB_N),
            0xde => Ok(Instruction::SBC_A_N),
            0xe8 => Ok(Instruction::ADD_SP_DD),
            // logic
            0xa0..=0xa5 | 0xa7 => Ok(Instruction::AND_R),
            0xa6 => Ok(Instruction::AND_HL),
            0xa8..=0xad | 0xaf => Ok(Instruction::XOR_R),
            0xae => Ok(Instruction::XOR_HL),
            0xb0..=0xb5 | 0xb7 => Ok(Instruction::OR_R),
            0xb6 => Ok(Instruction::OR_HL),
            0xe6 => Ok(Instruction::AND_N),
            0xee => Ok(Instruction::XOR_N),
            0xf6 => Ok(Instruction::OR_N),
            // compare
            0xb8..=0xbd | 0xbf => Ok(Instruction::CP_R),
            0xbe => Ok(Instruction::CP_HL),
            0xfe => Ok(Instruction::CP_N),
            // rotate
            0x07 => Ok(Instruction::RLCA),
            0x17 => Ok(Instruction::RLA),
            0x0f => Ok(Instruction::RRCA),
            0x1f => Ok(Instruction::RRA),
            // jump
            0x18 => Ok(Instruction::JR_PC_DD),
            0x20 | 0x30 | 0x28 | 0x38 => Ok(Instruction::JR_F_PC_DD),
            0xc2 | 0xd2 | 0xca | 0xda => Ok(Instruction::JP_F_NN),
            0xc3 => Ok(Instruction::JP_NN),
            0xe9 => Ok(Instruction::JP_HL),
            // call
            0xcd => Ok(Instruction::CALL_NN),
            0xc4 | 0xcc | 0xd4 | 0xdc => Ok(Instruction::CALL_F_NN),
            // ret
            0xc9 => Ok(Instruction::RET),
            0xc0 | 0xd0 | 0xc8 | 0xd8 => Ok(Instruction::RET_F),
            0xd9 => Ok(Instruction::RETI),
            // reset
            0xc7 | 0xd7 | 0xe7 | 0xf7 | 0xcf | 0xdf | 0xef | 0xff => Ok(Instruction::RES_R),

            0xf3 => Ok(Instruction::DI),
            0xfb => Ok(Instruction::EI),
            0xcb => Ok(Instruction::PREFIX),

            0x27 => Ok(Instruction::DAA),
            0x2f => Ok(Instruction::CPL),
            0x37 => Ok(Instruction::SCF),
            0x3f => Ok(Instruction::CCF),
            _ => Err(GBError::InstructionNotFound(inst)),
        }
    }

    pub fn function(&self) -> GBResult<InstructionFn> {
        match self {
            // 8bit load 
            Instruction::LD_R_R => Ok(dummy), // xx
            Instruction::LD_R_N => Ok(dummy), // xx nn
            Instruction::LD_R_HL => Ok(dummy), // xx
            Instruction::LD_HL_R => Ok(dummy), // 7x
            Instruction::LD_HL_N => Ok(dummy), // 36 nn
            Instruction::LD_A_BC => Ok(dummy), // 0x0a
            Instruction::LD_A_DE => Ok(dummy), //0x1a
            Instruction::LD_A_NN => Ok(dummy), // 0xfa
            Instruction::LD_BC_A => Ok(dummy), // 0x02
            Instruction::LD_DE_A => Ok(dummy), // 0x12
            Instruction::LD_NN_A => Ok(dummy), // 0xea
            Instruction::LD_A_IO_N => Ok(dummy), // 0xf0 nn
            Instruction::LD_IO_N_A => Ok(dummy), // 0xe0 nn
            Instruction::LD_A_IO_C => Ok(dummy), // 0xf2
            Instruction::LD_IO_C_A => Ok(dummy), // 0xe2
            Instruction::LDI_HL_A => Ok(dummy), // 0x22
            Instruction::LDI_A_HL => Ok(dummy), // 0x2a
            Instruction::LDD_HL_A => Ok(dummy), // 0x32
            Instruction::LDD_A_HL => Ok(dummy), //0x3a
            // 16bit load
            Instruction::LD_RR_NN => Ok(dummy), // 0xx1 nn nn
            Instruction::LD_SP_HL => Ok(dummy), // 0xf9
            Instruction::LD_NN_SP => Ok(dummy), // 0x08
            Instruction::PUSH_RR => Ok(dummy), // 0xx5
            Instruction::POP_RR => Ok(dummy), // 0xx1
            // 8bit arithmethic/logic
            Instruction::ADD_A_R => Ok(dummy), // 0x8x
            Instruction::ADD_A_N => Ok(dummy), // 0xc6 nn
            Instruction::ADD_A_HL => Ok(dummy), // 0x86
            Instruction::ADC_A_R => Ok(dummy), // 0x8x
            Instruction::ADC_A_N => Ok(dummy), // 0xce nn
            Instruction::ADC_A_HL => Ok(dummy), // 0x8e
            Instruction::SUB_R => Ok(dummy), // 0x9x
            Instruction::SUB_N => Ok(dummy), // 0xd6 nn
            Instruction::SUB_HL => Ok(dummy), // 0x96
            Instruction::SBC_A_R => Ok(dummy), // 0x9x
            Instruction::SBC_A_N => Ok(dummy), // 0xde nn
            Instruction::SBC_A_HL => Ok(dummy), // 0x9e
            Instruction::AND_R => Ok(dummy), // 0xax
            Instruction::AND_N => Ok(dummy), // 0xe6 nn
            Instruction::AND_HL => Ok(dummy), // 0xax
            Instruction::XOR_R => Ok(dummy), // 0xax
            Instruction::XOR_N => Ok(dummy), // 0xee nn
            Instruction::XOR_HL => Ok(dummy), // 0xae
            Instruction::OR_R => Ok(dummy), // 0xbx
            Instruction::OR_N => Ok(dummy), // 0xf6 nn
            Instruction::OR_HL => Ok(dummy), // 0xb6
            Instruction::CP_R => Ok(dummy), // 0xbx
            Instruction::CP_N => Ok(dummy), // 0xfe nn
            Instruction::CP_HL => Ok(dummy), // 0xbe
            Instruction::INC_R => Ok(dummy), // 0xxx
            Instruction::INC_HL => Ok(dummy), // 0x34
            Instruction::DEC_R => Ok(dummy), // 0xxx
            Instruction::DEC_HL => Ok(dummy), // 0x35
            Instruction::DAA => Ok(dummy), // 0x27
            Instruction::CPL => Ok(dummy), // 0x2f
            // 16bit arithmethic/logic
            Instruction::ADD_HL_RR => Ok(dummy), // 0xx9
            Instruction::INC_RR => Ok(dummy), // 0xx3
            Instruction::DEC_RR => Ok(dummy), // 0xxb
            Instruction::ADD_SP_DD => Ok(dummy), // 0xe8
            Instruction::LD_HL_SP_DD => Ok(dummy), // 0xf8
            Instruction::RLCA => Ok(dummy), // 0x07
            Instruction::RLA => Ok(dummy), // 0x17
            Instruction::RRCA => Ok(dummy), // 0x0f
            Instruction::RRA => Ok(dummy), // 0x1f
            Instruction::RLC_R => Ok(dummy), // 0xcb 0x
            Instruction::RLC_HL => Ok(dummy), // 0xcb 06
            Instruction::RL_R => Ok(dummy), // cb 1x
            Instruction::RL_HL => Ok(dummy), // cb 16
            Instruction::RRC_R => Ok(dummy), // cb 0x
            Instruction::RRC_HL => Ok(dummy), // cb 0e
            Instruction::RR_R => Ok(dummy), // cb 1x
            Instruction::RR_HL => Ok(dummy), // cb 1e
            Instruction::SLA_R => Ok(dummy), // cb 2x
            Instruction::SLA_HL => Ok(dummy), // cb 26
            Instruction::SWAP_R => Ok(dummy), // cb 3x
            Instruction::SWAP_HL => Ok(dummy), // cb 36
            Instruction::SRA_R => Ok(dummy), // cb 2x
            Instruction::SRA_HL => Ok(dummy), // cb 2e
            Instruction::SRL_R => Ok(dummy), // cb 3x
            Instruction::SRL_HL => Ok(dummy), // cb 3e
            // single bit operation
            Instruction::BIT_N_R => Ok(dummy), // cb xx
            Instruction::BIT_N_HL => Ok(dummy), // cb xx
            Instruction::SET_R => Ok(dummy), // cb xx
            Instruction::SET_HL => Ok(dummy), // cb xx
            Instruction::RES_R => Ok(dummy), // cb xx
            Instruction::RES_HL => Ok(dummy), // cb xx
            // cpu control
            Instruction::CCF => Ok(dummy), // 3f
            Instruction::SCF => Ok(dummy), // 37
            Instruction::NOP => Ok(dummy), // 0x00
            Instruction::HALT => Ok(dummy), // 76
            Instruction::STOP => Ok(dummy), // 10 00
            Instruction::DI => Ok(dummy), // f3
            Instruction::EI => Ok(dummy), // fb
            // jump
            Instruction::JP_NN => Ok(dummy), // c3 nn nn
            Instruction::JP_HL => Ok(dummy), // e9
            Instruction::JP_F_NN => Ok(dummy), // xx nn nn
            Instruction::JR_PC_DD => Ok(dummy), // 18 dd
            Instruction::JR_F_PC_DD => Ok(dummy), // xx dd
            Instruction::CALL_NN => Ok(dummy), // cd nn nn
            Instruction::CALL_F_NN => Ok(dummy), // xx nn nn
            Instruction::RET => Ok(dummy), // c9
            Instruction::RET_F => Ok(dummy), // xx
            Instruction::RETI => Ok(dummy), // d9
            Instruction::RST => Ok(dummy), // xx
            Instruction::PREFIX => Ok(dummy), // cb
        }
    }
}

fn op1(inst: u8) -> usize {
    (inst & 0b0000_0111) as usize
}

fn op2(inst: u8) -> usize {
    (inst & 0b0011_1000) as usize
}


pub fn dummy(inst: u8, reg: &mut Register, bus: &mut Bus) -> GBResult<usize> {
    Ok(0usize)
}

pub fn ld_r_r(inst: u8, reg: &mut Register, bus: &mut Bus) -> GBResult<usize> {
    let reg1 = op1(inst);
    let reg2 = op2(inst);
    Ok(0usize)
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use crate::cpu::register::Register;
    use crate::cpu::bus::Bus;
    use crate::mem::ram::Ram;
    use crate::mem::rom::Rom;
    use crate::cartridge::Cartridge;
    use crate::timer::Timer;
    #[test]
    fn test_instruction_from() {
        assert_eq!(Instruction::from(0x20).unwrap(), Instruction::JR_F_PC_DD);
        assert_eq!(Instruction::from(0x94).unwrap(), Instruction::SUB_R);
        assert_eq!(Instruction::from(0x86).unwrap(), Instruction::ADD_A_HL);
        assert_eq!(Instruction::from(0xc6).unwrap(), Instruction::ADD_A_N);
        assert_eq!(Instruction::from(0xf8).unwrap(), Instruction::LD_HL_SP_DD);
        assert_eq!(Instruction::from(0x07).unwrap(), Instruction::RLCA);
        assert_eq!(Instruction::from(0x69).unwrap(), Instruction::LD_R_R);
        assert_eq!(Instruction::from(0x0b).unwrap(), Instruction::DEC_RR);
        assert_eq!(Instruction::from(0x29).unwrap(), Instruction::ADD_HL_RR);
        assert_eq!(Instruction::from(0x34).unwrap(), Instruction::INC_HL);
        assert_eq!(Instruction::from(0xe8).unwrap(), Instruction::ADD_SP_DD);
        assert_eq!(Instruction::from(0xf8).unwrap(), Instruction::LD_HL_SP_DD);
        assert_eq!(Instruction::from(0xf0).unwrap(), Instruction::LD_A_IO_N);
        assert_eq!(Instruction::from(0xc1).unwrap(), Instruction::POP_RR);
        assert_eq!(Instruction::from(0xe5).unwrap(), Instruction::PUSH_RR);
        assert_eq!(Instruction::from(0xf9).unwrap(), Instruction::LD_SP_HL);
        assert_eq!(Instruction::from(0xca).unwrap(), Instruction::JP_F_NN);
        assert_eq!(Instruction::from(0xc3).unwrap(), Instruction::JP_NN);
        assert_eq!(Instruction::from(0xe9).unwrap(), Instruction::JP_HL);
        assert_eq!(Instruction::from(0xc0).unwrap(), Instruction::RET_F);
        assert_eq!(Instruction::from(0xc9).unwrap(), Instruction::RET);
        assert_eq!(Instruction::from(0xd9).unwrap(), Instruction::RETI);
        assert_eq!(Instruction::from(0xb9).unwrap(), Instruction::CP_R);
        assert_eq!(Instruction::from(0xe6).unwrap(), Instruction::AND_N);
        assert_eq!(Instruction::from(0xcd).unwrap(), Instruction::CALL_NN);
        assert_eq!(Instruction::from(0xf3).unwrap(), Instruction::DI);
        assert_eq!(Instruction::from(0xcd).unwrap(), Instruction::CALL_NN);
        assert_eq!(Instruction::from(0xfa).unwrap(), Instruction::LD_A_NN);
        assert_eq!(Instruction::from(0x01).unwrap(), Instruction::LD_RR_NN);
        assert_eq!(Instruction::from(0x08).unwrap(), Instruction::LD_NN_SP);
    }
    #[test]
    fn test_instruction_function() {
        let mut reg = Register::new();
        let mut ram = Ram::new(Vec::new());
        let mut hram = Ram::new(Vec::new());
        let cart_rom = Rom::new(Vec::new());
        let mut cart_ram = Ram::new(Vec::new());
        let mut cart = Cartridge::new(&cart_rom, &mut cart_ram, false, false);
        let mut timer = Timer::new();
        let mut bus = Bus::new(&mut ram, &mut hram, &mut cart, &mut timer);
        let inst = Instruction::NOP;
        let func = inst.function().unwrap();
        let res = func(0u8, &mut reg, &mut bus).is_ok();
        assert_eq!(true, true)
    }
}
