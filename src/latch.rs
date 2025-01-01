use crate::gate;

pub struct SRLatch {
    q: bool,
    qn: bool,
}

impl SRLatch {
    /// Creates a new SR latch in the reset state.
    pub fn new() -> Self {
        SRLatch { q: false, qn: true }
    }

    /// Set the set and reset inputs
    pub fn set(&mut self, s: bool, r: bool) {
        if s && r {
            panic!("restricted combination");
        }

        self.qn = gate::nor(&[self.q, s]);
        self.q = gate::nor(&[r, self.qn]);
    }

    /// Alternative set function using nand gates
    pub fn set_nand(&mut self, s: bool, r: bool) {
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

    pub fn qn(&self) -> bool {
        self.qn
    }
}

impl Default for SRLatch {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DLatch {
    sr_latch: SRLatch,
}

impl DLatch {
    /// Creates a new D latch in the reset state.
    pub fn new() -> Self {
        DLatch {
            sr_latch: SRLatch::new(),
        }
    }

    /// Set the enable and data inputs
    pub fn set(&mut self, e: bool, d: bool) {
        let dn = gate::not(&d);
        let s = gate::and(&[d, e]);
        let r = gate::and(&[dn, e]);
        self.sr_latch.set(s, r);
    }

    pub fn q(&self) -> bool {
        self.sr_latch.q()
    }

    pub fn qn(&self) -> bool {
        self.sr_latch.qn()
    }
}

impl Default for DLatch {
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

            // Send test signals
            latch.set(s, r);

            assert_eq!(latch.q(), q_expected, "failed for inputs: {:?}", (s, r))
        }
    }

    #[test]
    fn test_d_latch() {
        for (q_init, e, d, q_expected) in [
            // Hold state
            (false, false, false, false),
            (true, false, false, true),
            (false, false, true, false),
            (true, false, true, true),
            // Reset
            (false, true, false, false),
            (true, true, false, false),
            // Set
            (false, true, true, true),
            (true, true, true, true),
        ] {
            // Set up initial state
            let mut latch = DLatch {
                sr_latch: SRLatch {
                    q: q_init,
                    qn: !q_init,
                },
            };

            // Send test signals
            latch.set(e, d);

            assert_eq!(latch.q(), q_expected, "failed for inputs: {:?}", (e, d))
        }
    }
}
