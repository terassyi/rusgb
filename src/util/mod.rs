
pub fn u8_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) | b as u16
}

pub fn split_u16(val: u16) -> (u8, u8) {
    ((val >> 8) as u8, val as u8)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_u8_to_u16() {
        let a: u8 = 0b1011_1100;
        let b: u8 = 0b0001_0100;
        let want: u16 = 0b1011_1100_0001_0100;
        assert_eq!(super::u8_to_u16(a, b), want);
    }
    #[test]
    fn test_split_u16() {
        let a: u8 = 0b1011_1100;
        let b: u8 = 0b0001_0100;
        let val: u16 = 0b1011_1100_0001_0100;
        assert_eq!(super::split_u16(val), (a, b));
    }
}
