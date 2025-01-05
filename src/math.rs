use crate::gate::{and, or, xor};

/// Perform a half add operation. Returns the sum and carry bits
pub fn half_add(a: bool, b: bool) -> (bool, bool) {
    (xor(&[a, b]), and(&[a, b]))
}

/// Perform a full add operation. Returns the sum and carry bits
pub fn full_add(a: bool, b: bool, cin: bool) -> (bool, bool) {
    (
        xor(&[xor(&[a, b]), cin]),
        or(&[and(&[xor(&[a, b]), cin]), and(&[a, and(&[a, b])])]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_add() {
        for (a, b, sum, carry) in [
            (false, false, false, false),
            (false, true, true, false),
            (true, false, true, false),
            (true, true, false, true),
        ] {
            assert_eq!(
                half_add(a, b),
                (sum, carry),
                "failed for inputs: {:?}",
                (a, b)
            );
        }
    }

    #[test]
    fn test_full_add() {
        for (a, b, cin, sum, carry) in [
            (false, false, false, false, false),
            (false, false, true, true, false),
            (false, true, false, true, false),
            (false, true, true, false, true),
            (true, false, false, true, false),
            (true, false, true, false, true),
            (true, true, false, false, true),
            (true, true, true, true, true),
        ] {
            assert_eq!(
                full_add(a, b, cin),
                (sum, carry),
                "failed for inputs: {:?}",
                (a, b, cin)
            );
        }
    }
}
