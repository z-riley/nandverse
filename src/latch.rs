use crate::gate;

pub struct SRLatch {
    q: bool,
    qn: bool,
}

impl SRLatch {
    /// Creates a new SR latch in the reset state.
    pub fn new() -> SRLatch {
        SRLatch { q: false, qn: true }
    }

    pub fn set(&mut self, s: bool, r: bool) {
        if s && r {
            panic!("restricted combination");
        }

        if s {
            // Qn must be evaluated first if set is high
            self.q = gate::nand(&[gate::not(&s), self.qn]);
            self.qn = gate::nand(&[self.q, gate::not(&r)]);
        } else if r {
            // Q must be evaluated first if reset is high
            self.qn = gate::nand(&[self.q, gate::not(&r)]);
            self.q = gate::nand(&[gate::not(&s), self.qn]);
        }
    }

    pub fn q(&self) -> bool {
        self.q
    }
}

impl Default for SRLatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sr_latch() {
        for (q_init, s, r, q_expected) in [
            // Hold state
            (false, false, false, false),
            (true, false, false, true),
            // Reset
            (false, false, true, false),
            (true, false, true, false),
            // Set
            (false, true, false, true),
            (true, true, false, true),
        ] {
            // Set up initial state
            let mut latch = SRLatch {
                q: q_init,
                qn: !q_init,
            };

            dbg!(q_init, s, r);

            // Send test signals
            latch.set(s, r);

            assert_eq!(latch.q(), q_expected, "failed for inputs: {:?}", (s, r))
        }
    }
}
