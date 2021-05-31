
#[derive(Debug)]
pub struct Timer {
    tima: u8, // timer counter
    tma: u8, // timer modulo
    tac: u8, // timer control 3bit value bit 2 => timer stop, bit 1-0: input clock select
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            tima: 0u8,
            tma: 0u8,
            tac: 0u8,
        }
    }
}
