use crate::gate::{and, or, xor};

/// Performs an XOR operation on two N bit words.
pub const fn xor_n<const N: usize>(a: &[bool; N], b: &[bool; N]) -> [bool; N] {
    let mut result = [false; N];
    let mut i = 0;
    while i < a.len() {
        result[i] = xor(&[a[i], b[i]]);
        i += 1;
    }
    result
}

/// Performs an OR operation on two N bit words.
pub const fn or_n<const N: usize>(a: &[bool; N], b: &[bool; N]) -> [bool; N] {
    let mut result = [false; N];
    let mut i = 0;
    while i < a.len() {
        result[i] = or(&[a[i], b[i]]);
        i += 1;
    }
    result
}

/// Performs an AND operation on two N bit words.
pub const fn and_n<const N: usize>(a: &[bool; N], b: &[bool; N]) -> [bool; N] {
    let mut result = [false; N];
    let mut i = 0;
    while i < a.len() {
        result[i] = and(&[a[i], b[i]]);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::bus::u8_to_bus;

    use super::*;

    #[test]
    fn test_xor_n() {
        assert_eq!(
            xor_n(&u8_to_bus(0b0101_0110), &u8_to_bus(0b0011_0010)),
            u8_to_bus(0b0110_0100),
        )
    }

    #[test]
    fn test_or_n() {
        assert_eq!(
            or_n(&u8_to_bus(0b0101_1100), &u8_to_bus(0b0000_1000)),
            u8_to_bus(0b0101_1100),
        )
    }

    #[test]
    fn test_and_n() {
        assert_eq!(
            and_n(&u8_to_bus(0b0111_0010), &u8_to_bus(0b0001_0011)),
            u8_to_bus(0b0001_0010),
        )
    }
}
