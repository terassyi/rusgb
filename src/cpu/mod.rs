mod register;
mod instruction;

use register::*;

pub struct Cpu<'a> {
    register: &'a mut Register,
}
