use crate::gate::{and, not, or};

/// Returns the input bit corresponding to the select value
pub fn mux2(select: bool, input: &[bool; 2]) -> bool {
    or(&[and(&[not(select), input[0]]), and(&[select, input[1]])])
}

/// Returns the input bit corresponding to the select value (little-endian)
pub fn mux4(select: &[bool; 2], input: &[bool; 4]) -> bool {
    or(&[
        and(&[input[0], not(select[0]), not(select[1])]),
        and(&[input[1], select[0], not(select[1])]),
        and(&[input[2], not(select[0]), select[1]]),
        and(&[input[3], select[0], select[1]]),
    ])
}

/// Returns the input bit corresponding to the select value (little-endian)
pub fn mux8(select: &[bool; 3], input: &[bool; 8]) -> bool {
    or(&[
        and(&[input[0], not(select[0]), not(select[1]), not(select[2])]),
        and(&[input[1], select[0], not(select[1]), not(select[2])]),
        and(&[input[2], not(select[0]), select[1], not(select[2])]),
        and(&[input[3], select[0], select[1], not(select[2])]),
        and(&[input[4], not(select[0]), not(select[1]), select[2]]),
        and(&[input[5], select[0], not(select[1]), select[2]]),
        and(&[input[6], not(select[0]), select[1], select[2]]),
        and(&[input[7], select[0], select[1], select[2]]),
    ])
}

/// Returns the input bit corresponding to the select value (little-endian)
pub fn mux16(select: &[bool; 4], input: &[bool; 16]) -> bool {
    mux2(
        select[3],
        &[
            mux8(
                select[..3].try_into().unwrap(),
                input[0..8].try_into().unwrap(),
            ),
            mux8(
                select[..3].try_into().unwrap(),
                input[8..16].try_into().unwrap(),
            ),
        ],
    )
}

/// Returns the input bit corresponding to the select value (little-endian)
pub fn mux32(select: &[bool; 5], input: &[bool; 32]) -> bool {
    mux2(
        select[4],
        &[
            mux16(
                select[..4].try_into().unwrap(),
                input[0..16].try_into().unwrap(),
            ),
            mux16(
                select[..4].try_into().unwrap(),
                input[16..].try_into().unwrap(),
            ),
        ],
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bus;

    #[test]
    fn test_mux2() {
        for (select, input, expect) in [
            (false, [false, true], false),
            (false, [true, false], true),
            (true, [false, false], false),
            (true, [false, true], true),
        ] {
            assert_eq!(
                mux2(select, &input),
                expect,
                "failed for inputs: {:?}",
                (select, input)
            )
        }
    }

    #[test]
    fn test_mux4() {
        for (select, input, expect) in [
            ([false, false], [true, false, false, false], true),
            ([true, false], [false, true, false, false], true),
            ([false, true], [true, true, false, true], false),
            ([true, true], [false, false, false, true], true),
        ] {
            assert_eq!(
                mux4(&select, &input),
                expect,
                "failed for inputs: {:?}",
                (select, input)
            )
        }
    }

    #[test]
    fn test_mux8() {
        for (select, input, expect) in [
            ([false, false, false], bus::u8_to_bus(0b0000_0000), false),
            ([true, false, false], bus::u8_to_bus(0b0000_0010), true),
            ([false, true, false], bus::u8_to_bus(0b0000_0100), true),
            ([false, true, false], bus::u8_to_bus(0b0000_1000), false),
        ] {
            assert_eq!(
                mux8(&select, &input),
                expect,
                "failed for inputs: {:?}",
                (select, input)
            )
        }
    }

    #[test]
    fn test_mux16() {
        for (select, input, expect) in [
            (
                [false, false, false, false],
                bus::u16_to_bus(0b0000_0000_0000_0000),
                false,
            ),
            (
                [true, false, false, false],
                bus::u16_to_bus(0b0000_0000_0000_0010),
                true,
            ),
            (
                [false, true, false, false],
                bus::u16_to_bus(0b0000_0000_0000_0100),
                true,
            ),
            (
                [false, true, false, false],
                bus::u16_to_bus(0b0000_0000_0000_1000),
                false,
            ),
            (
                [false, false, false, true],
                bus::u16_to_bus(0b0000_0001_0000_0000),
                true,
            ),
            (
                [true, true, true, true],
                bus::u16_to_bus(0b1000_0000_0000_0010),
                true,
            ),
            (
                [true, true, true, true],
                bus::u16_to_bus(0b0000_0000_0000_0000),
                false,
            ),
        ] {
            assert_eq!(
                mux16(&select, &input),
                expect,
                "failed for inputs: {:?}",
                (select, input)
            )
        }
    }

    #[test]
    fn test_mux32() {
        for (select, input, expect) in [
            (
                [false, false, false, false, false],
                bus::u32_to_bus(0b0000_0000_0000_0000_0000_0000_0000_0000),
                false,
            ),
            (
                [true, false, false, false, false],
                bus::u32_to_bus(0b0000_0000_0000_0000_0000_0000_0000_0010),
                true,
            ),
            (
                [false, false, false, true, false],
                bus::u32_to_bus(0b0000_0000_0000_0000_0000_0001_0000_0000),
                true,
            ),
            (
                [false, false, false, true, true],
                bus::u32_to_bus(0b0000_0001_0000_0000_0000_0000_0000_0000),
                true,
            ),
            (
                [true, true, true, true, true],
                bus::u32_to_bus(0b0111_1111_1111_1111_1111_1111_1111_1111),
                false,
            ),
        ] {
            assert_eq!(
                mux32(&select, &input),
                expect,
                "failed for inputs: {:?}",
                (select, input)
            )
        }
    }
}
