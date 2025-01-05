use crate::flipflop::DFlipflop;

/// Asynchronous counter of N bits in width
pub struct RippleCounter<const N: usize> {
    flipflops: [DFlipflop; N],
}

impl<const N: usize> RippleCounter<N> {
    pub fn new() -> Self {
        let mut counter = RippleCounter {
            flipflops: core::array::from_fn(|_| DFlipflop::new()),
        };

        counter.init();
        counter
    }

    /// Update the counter with a new input
    pub fn update(&mut self, clk: bool) {
        // Feed the clock signal into the LSB flip-flop
        self.flipflops[0].update(clk, self.flipflops[0].qn());

        // Chain the inverted output through the adjacent flip-flops
        for i in 1..self.flipflops.len() {
            self.flipflops[i].update(self.flipflops[i - 1].qn(), self.flipflops[i].qn());
        }
    }

    /// Clear the value of the counter
    pub fn clear(&mut self) {
        for i in 0..self.flipflops.len() {
            self.flipflops[i].clear();
        }
        self.init();
    }

    fn init(&mut self) {
        // Set the clock to false to avoid the race condition that occurs when setting D and CLK
        // high simultaneously
        self.update(false);
    }

    // FIXME: MAKE THIS WORK FOR ALL WHOLE NUMBERS
    pub fn value<T: From<u64>>(&self) -> T {
        let mut val = 0u64;
        for (i, ff) in self.flipflops.iter().enumerate() {
            let q = ff.q();
            val |= if q { 1 << i } else { 0 };
        }

        val.into()
    }
}

impl<const N: usize> Default for RippleCounter<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ripple_counter() {
        const WIDTH: usize = 8;
        let mut counter: RippleCounter<8> = RippleCounter::<WIDTH>::new();
        assert_eq!(counter.value::<u64>(), 0);

        // Count up to a number within the range of a counter
        let num_toggles = 100;
        for _ in 0..num_toggles {
            counter.update(true);
            counter.update(false);
        }
        assert_eq!(counter.value::<u64>(), num_toggles);

        // Clear the counter
        counter.clear();
        assert_eq!(counter.value::<u64>(), 0);

        // Count up to a number above the capacity of the counter. The value should overflow
        let num_toggles = 300;
        for _ in 0..num_toggles {
            counter.update(true);
            counter.update(false);
        }
        let max_count: u64 = 2u64.pow(WIDTH as u32);
        assert_eq!(counter.value::<u64>(), num_toggles % max_count);

        let a = counter.value::<u32>();
    }
}
