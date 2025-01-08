use crate::gate::{and, not, or};

/// Returns the input bit corresponding to the select value
pub fn mux2(select: bool, input: &[bool; 2]) -> bool {
    or(&[and(&[not(select), input[0]]), and(&[select, input[1]])])
}

/// Returns the input but corresponding to the select value (little-endian)
pub fn mux4(select: &[bool; 2], input: &[bool; 4]) -> bool {
    or(&[
        and(&[input[0], not(select[0]), not(select[1])]),
        and(&[input[1], select[0], not(select[1])]),
        and(&[input[2], not(select[0]), select[1]]),
        and(&[input[3], select[0], select[1]]),
    ])
}

/// Returns the input but corresponding to the select value (little-endian)
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

/// Returns the input but corresponding to the select value (little-endian)
pub fn mux16(select: &[bool; 4], input: &[bool; 16]) -> bool {
    // Chain together mux2 and mux8
    todo!()
}

/// Returns the input but corresponding to the select value (little-endian)
pub fn mux32(select: &[bool; 5], input: &[bool; 32]) -> bool {
    // Chain together mux4 and mux8
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

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
                input
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
                input
            )
        }
    }

    #[test]
    fn test_mux8() {
        for (select, input, expect) in [] {
            todo!();
            assert_eq!(
                mux8(&select, &input),
                expect,
                "failed for inputs: {:?}",
                input
            )
        }
    }
}
