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

pub struct RippleCarryAdder<const N: usize> {}

impl<const N: usize> RippleCarryAdder<N> {
    pub fn new() -> Self {
        RippleCarryAdder {}
    }

    pub fn add(&self, a: u64, b: u64) -> u64 {
        let mut sum = 0u64;
        let mut carry = false;
        for i in 0..N {
            let a_bit = (a >> i) & 1 == 1;
            let b_bit = (b >> i) & 1 == 1;
            let (s, cout) = full_add(a_bit, b_bit, carry);
            carry = cout;
            sum |= if s { 1 << i } else { 0 };
        }
        sum
    }
}

impl<const N: usize> Default for RippleCarryAdder<N> {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_ripple_carry_add() {
        for (a, b, expect) in [
            (0, 0, 0),
            (1, 1, 2),
            (255, 0, 255),
            (255, 2, 1), // overflow
        ] {
            let adder = RippleCarryAdder::<8>::new();
            assert_eq!(adder.add(a, b), expect, "failed for inputs: {:?}", (a, b));
        }
    }
}
