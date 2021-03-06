pub mod rom;
pub mod ram;

pub const ROM_ADDR_TOP: usize = 0x0000;
pub const ROM_ADDR_TAIL: usize = 0x3fff;
pub const ROM_BANK_ADDR_TOP: usize = 0x4000;
pub const ROM_BANK_ADDR_TAIL: usize = 0x7fff;
pub const VRAM_ADDR_TOP: usize = 0x8000;
pub const VRAM_ADDR_TAIL: usize = 0x9fff;
pub const WRAM_BANK_0_ADDR_TOP: usize = 0xc000;
pub const WRAM_BANK_0_ADDR_TAIL: usize = 0xcfff;
pub const WRAM_BANK_1_ADDR_TOP: usize = 0xd000;
pub const WRAM_BANK_1_ADDR_TAIL: usize = 0xdfff;
pub const ECHO_RAM_ADDR_TOP: usize = 0xe000;
pub const ECHO_RAM_ADDR_TAIL: usize = 0xfdff;
pub const OAM_ADDR_TOP: usize = 0xfe00;
pub const OAM_ADDR_TAIL: usize = 0xfe9f;
pub const IO_PORTS_ADDR_TOP: usize = 0xff00;
pub const IO_PORTS_ADDR_TAIL: usize = 0xff7f;
pub const HRAM_ADDR_TOP: usize = 0xff80;
pub const HRAM_ADDR_TAIL: usize = 0xfffe;
pub const INTERRUPT_ENABLE_REG_ADDR: usize = 0xffff;
