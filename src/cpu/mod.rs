mod register;
mod bus;
mod instruction;

use register::*;
use bus::*;

pub struct Cpu<'a> {
    register: &'a mut Register,
    bus: &'a mut Bus<'a>,
}
