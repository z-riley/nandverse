use crate::gate;
use crate::latch;

/// Edge-triggered D flip-flop
pub struct DFlipflop {
    master: latch::DLatch,
    slave: latch::DLatch,
}

impl DFlipflop {
    /// Creates a new D flip-flop in the reset state.
    pub fn new() -> Self {
        DFlipflop {
            master: latch::DLatch::new(),
            slave: latch::DLatch::new(),
        }
    }

    /// Updates the flip-flop based on new inputs. The flip-flop triggers on the rising edge of the
    /// clock.
    pub fn update(&mut self, clk: bool, d: bool) {
        self.master.set(gate::not(&clk), d);
        self.slave.set(clk, self.master.q());
    }

    pub fn q(&self) -> bool {
        self.slave.q()
    }

    pub fn qn(&self) -> bool {
        self.slave.qn()
    }
}

impl Default for DFlipflop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_flipflop() {
        // Reference timing diagram:
        // https://www.build-electronic-circuits.com/wp-content/uploads/2022/11/clock-4.png

        let mut flipflop = DFlipflop::new();

        // Start with Q low
        let mut expect_q = false;
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge and D low
        let mut clk = true;
        let mut d = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Keep the clock high and set D high
        d = true;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge and keep D high
        clk = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge and keep D high
        clk = true;
        flipflop.update(clk, d);
        expect_q = true;
        assert_eq!(flipflop.q(), expect_q);

        // Send clock falling edge and D low
        clk = false;
        d = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);

        // Send clock rising edge
        clk = true;
        expect_q = false;
        flipflop.update(clk, d);
        assert_eq!(flipflop.q(), expect_q);
    }
}
