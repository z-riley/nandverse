use crate::mux;

pub fn logical_shift_right_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    [
        mux::mux8(shift, value),
        mux::mux8(
            shift,
            &[
                value[1], value[2], value[3], value[4], value[5], value[6], value[7], value[0],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[2], value[3], value[4], value[5], value[6], value[7], value[0], value[1],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[3], value[4], value[5], value[6], value[7], value[0], value[1], value[2],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[4], value[5], value[6], value[7], value[0], value[1], value[2], value[3],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[5], value[6], value[7], value[0], value[1], value[1], value[2], value[3],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[6], value[7], value[0], value[1], value[2], value[3], value[4], value[5],
            ],
        ),
        mux::mux8(
            shift,
            &[
                value[7], value[0], value[1], value[2], value[3], value[4], value[5], value[6],
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::u8_to_bus;

    #[test]
    fn test_left_shift() {
        for (shift, value, expected) in [
            (
                [false, false, false],
                u8_to_bus(0b0000_0000),
                u8_to_bus(0b0000_0000),
            ),
            (
                [true, false, false],
                u8_to_bus(0b0000_0100),
                u8_to_bus(0b0000_0010),
            ),
            (
                [false, true, false],
                u8_to_bus(0b0000_0010),
                u8_to_bus(0b1000_0000),
            ),
        ] {
            assert_eq!(
                logical_shift_right_8(&shift, &value),
                expected,
                "failed for inputs: {:?}",
                (shift, value),
            )
        }
    }
}
