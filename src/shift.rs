use crate::mux;

pub fn logical_shift_right_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    let mut output = [false; 8];
    (0..8).for_each(|i| {
        dbg!(i);
        let mut input = [false; 8];
        (0..8).for_each(|j| {
            let n = (i + j) % 8;
            input[j] = value[n];
        });
        output[i] = mux::mux8(shift, &input);
        dbg!(input);
    });

    output
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
