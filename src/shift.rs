use crate::mux;

/// Shifts the bits in value left by shift amount. Replacement bits are all false
pub fn logical_shift_left_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    const WIDTH: usize = 8;

    // Connect the input to each mux at compile time
    const fn connect_input<const N: usize>(value: &[bool; N], i: usize) -> [bool; N] {
        let mut input = [false; N];
        let mut j = 0;
        while j < N {
            if i != 0 {
                input[j] = value[(N + i - j) % N];
            }
            j += 1;
        }
        input
    }

    let mut output = [false; WIDTH];
    (0..WIDTH).for_each(|i| {
        output[i] = mux::mux8(shift, &connect_input(value, i));
    });

    output
}

/// Shifts the bits in value right by shift amount. Replacement bits are all false
pub fn logical_shift_right_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    const WIDTH: usize = 8;

    // Connect the input to each mux at compile time
    const fn connect_input<const N: usize>(value: &[bool; N], i: usize) -> [bool; N] {
        let mut input = [false; N];
        let mut j = 0;
        while j < N {
            if i != N - 1 {
                input[j] = value[(j + i) % N];
            }
            j += 1;
        }
        input
    }

    let mut output = [false; WIDTH];
    (0..WIDTH).for_each(|i| {
        output[i] = mux::mux8(shift, &connect_input(value, i));
    });

    output
}

/// Shifts the bits in value left by shift amount. Bits shifted off of value are concatinated to
/// the other side of value
pub fn rotate_left_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    const WIDTH: usize = 8;

    // Connect the input to each mux at compile time
    const fn connect_input<const N: usize>(value: &[bool; N], i: usize) -> [bool; N] {
        let mut input = [false; N];
        let mut j = 0;
        while j < N {
            input[j] = value[(N + i - j) % N];
            j += 1;
        }
        input
    }

    let mut output = [false; WIDTH];
    (0..WIDTH).for_each(|i| {
        output[i] = mux::mux8(shift, &connect_input(value, i));
    });

    output
}

/// Shifts the bits in value right by shift amount. Bits shifted off of value are concatinated to
/// the other side of value
pub fn rotate_right_8(shift: &[bool; 3], value: &[bool; 8]) -> [bool; 8] {
    const WIDTH: usize = 8;

    // Connect the input to each mux at compile time
    const fn connect_input<const N: usize>(value: &[bool; N], i: usize) -> [bool; N] {
        let mut input = [false; N];
        let mut j = 0;
        while j < N {
            input[j] = value[(i + j) % N];
            j += 1;
        }
        input
    }

    let mut output = [false; WIDTH];
    (0..WIDTH).for_each(|i| {
        output[i] = mux::mux8(shift, &connect_input(value, i));
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::u8_to_bus;

    #[test]
    fn test_logical_shift_left_8() {
        for (shift, value, expected) in [
            (
                [false, false, false],
                u8_to_bus(0b0000_0000),
                u8_to_bus(0b0000_0000),
            ),
            (
                [true, false, false],
                u8_to_bus(0b0000_0100),
                u8_to_bus(0b0000_1000),
            ),
            (
                [false, true, false],
                u8_to_bus(0b0100_0000),
                u8_to_bus(0b0000_0000),
            ),
            ([false, false, true], u8_to_bus(1), u8_to_bus(16)),
        ] {
            assert_eq!(
                logical_shift_left_8(&shift, &value),
                expected,
                "failed for inputs: {:?}",
                (shift, value),
            )
        }
    }

    #[test]
    fn test_logical_shift_right_8() {
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
                u8_to_bus(0b0000_0000),
            ),
            ([false, false, true], u8_to_bus(16), u8_to_bus(1)),
        ] {
            assert_eq!(
                logical_shift_right_8(&shift, &value),
                expected,
                "failed for inputs: {:?}",
                (shift, value),
            )
        }
    }

    #[test]
    fn test_rotate_left_8() {
        for (shift, value, expected) in [
            (
                [false, false, false],
                u8_to_bus(0b0000_0000),
                u8_to_bus(0b0000_0000),
            ),
            (
                [true, false, false],
                u8_to_bus(0b0000_0100),
                u8_to_bus(0b0000_1000),
            ),
            (
                [false, true, false],
                u8_to_bus(0b0100_0000),
                u8_to_bus(0b0000_0001),
            ),
        ] {
            assert_eq!(
                rotate_left_8(&shift, &value),
                expected,
                "failed for inputs: {:?}",
                (shift, value),
            )
        }
    }

    #[test]
    fn test_rotate_right_8() {
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
                rotate_right_8(&shift, &value),
                expected,
                "failed for inputs: {:?}",
                (shift, value),
            )
        }
    }
}
