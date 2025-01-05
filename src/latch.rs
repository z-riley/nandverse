use crate::gate::{and, nand, not};

/// Active high SR latch with the following truth table:
///
/// | S | R | Q |
/// | - | - | - |
/// | 0 | 0 | Q |
/// | 0 | 1 | 0 |
/// | 1 | 0 | 1 |
/// | 1 | 1 | X |
#[derive(Clone, Copy, Debug)]
pub struct SRLatchActiveHigh {
    sr_latch_active_low: SRLatchActiveLow,
}

impl SRLatchActiveHigh {
    /// Creates a new SR latch in the reset state
    pub fn new() -> Self {
        SRLatchActiveHigh {
            sr_latch_active_low: SRLatchActiveLow::new(),
        }
    }

    /// Set the set and reset inputs
    pub fn set(&mut self, s: bool, r: bool) {
        self.sr_latch_active_low.set(not(s), not(r))
    }

    pub fn clear(&mut self) {
        self.sr_latch_active_low.clear();
    }

    pub fn q(&self) -> bool {
        self.sr_latch_active_low.q()
    }

    pub fn qn(&self) -> bool {
        self.sr_latch_active_low.qn()
    }
}

impl Default for SRLatchActiveHigh {
    fn default() -> Self {
        Self::new()
    }
}

/// Active low SR latch with the following truth table:
///
/// | S | R | Q |
/// | - | - | - |
/// | 0 | 0 | ? |
/// | 0 | 1 | 1 |
/// | 1 | 0 | 0 |
/// | 1 | 1 | Q |
#[derive(Clone, Copy, Debug)]
pub struct SRLatchActiveLow {
    q: bool,
    qn: bool,
}

impl SRLatchActiveLow {
    /// Creates a new SR latch in the reset state
    pub fn new() -> Self {
        SRLatchActiveLow { q: false, qn: true }
    }

    /// Set the set and reset inputs
    pub fn set(&mut self, s: bool, r: bool) {
        assert!(s | r, "restricted combination");

        // Propagate the signal from the first affected gate
        if !s {
            self.q = nand(&[s, self.qn]);
            self.qn = nand(&[self.q, r]);
        } else if !r {
            self.qn = nand(&[self.q, r]);
            self.q = nand(&[s, self.qn]);
        }
    }

    pub fn clear(&mut self) {
        self.q = false;
        self.qn = true;
    }

    pub fn q(&self) -> bool {
        self.q
    }

    pub fn qn(&self) -> bool {
        self.qn
    }
}

impl Default for SRLatchActiveLow {
    fn default() -> Self {
        Self::new()
    }
}

// Gated active high SR latch
#[derive(Clone, Copy)]
pub struct GatedSRLatch {
    sr_latch: SRLatchActiveHigh,
}

impl GatedSRLatch {
    /// Creates a new gated SR latch in the reset state
    pub fn new() -> Self {
        GatedSRLatch {
            sr_latch: SRLatchActiveHigh::new(),
        }
    }

    /// Set the set, enable, and reset inputs
    pub fn set(&mut self, s: bool, e: bool, r: bool) {
        self.sr_latch.set(and(&[s, e]), and(&[r, e]))
    }

    pub fn q(&self) -> bool {
        self.sr_latch.q()
    }

    pub fn qn(&self) -> bool {
        self.sr_latch.qn()
    }
}

impl Default for GatedSRLatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Active high D latch with the following truth table:
///
/// | E | D | Q |
/// | - | - | - |
/// | 0 | 0 | Q |
/// | 0 | 1 | Q |
/// | 1 | 0 | 0 |
/// | 1 | 1 | 1 |
#[derive(Debug)]
pub struct DLatch {
    sr_latch: SRLatchActiveHigh,
}

impl DLatch {
    /// Creates a new D latch in the reset state.
    pub fn new() -> Self {
        DLatch {
            sr_latch: SRLatchActiveHigh::new(),
        }
    }

    /// Set the enable and data inputs
    pub fn set(&mut self, e: bool, d: bool) {
        self.sr_latch.set(and(&[d, e]), and(&[not(d), e]));
    }

    pub fn clear(&mut self) {
        self.sr_latch.clear();
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
    fn test_sr_latch_active_high() {
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
            let mut latch = SRLatchActiveHigh {
                sr_latch_active_low: SRLatchActiveLow {
                    q: q_init,
                    qn: !q_init,
                },
            };

            latch.set(s, r);

            assert_eq!(
                latch.q(),
                q_expected,
                "failed for inputs: {:?}",
                (q_init, s, r)
            )
        }
    }

    #[test]
    fn test_sr_latch_active_low() {
        for (q_init, s, r, q_expected) in [
            // Hold state
            (false, true, true, false),
            (true, true, true, true),
            // Reset
            (false, true, false, false),
            (true, true, false, false),
            // Set
            (false, false, true, true),
            (true, false, true, true),
        ] {
            let mut latch = SRLatchActiveLow {
                q: q_init,
                qn: !q_init,
            };

            latch.set(s, r);

            assert_eq!(
                latch.q(),
                q_expected,
                "failed for inputs: {:?}",
                (q_init, s, r)
            );

            assert_eq!(
                latch.qn(),
                !q_expected,
                "failed for inputs: {:?}",
                (q_init, s, r)
            )
        }
    }

    #[test]
    fn test_gated_sr_latch() {
        for (q_init, s, e, r, q_expected) in [
            // Hold state (enabled)
            (false, false, true, false, false),
            (true, false, true, false, true),
            // Reset (enabled)
            (false, false, true, true, false),
            (true, false, true, true, false),
            // Set (enabled)
            (false, true, true, false, true),
            (true, true, true, false, true),
            // Hold state (disabled)
            (false, false, false, false, false),
            (true, false, false, false, true),
            // Reset (disabled)
            (false, false, false, true, false),
            (true, false, false, true, true),
            // Set (disabled)
            (false, true, false, false, false),
            (true, true, false, false, true),
        ] {
            let mut latch = GatedSRLatch {
                sr_latch: SRLatchActiveHigh {
                    sr_latch_active_low: SRLatchActiveLow {
                        q: q_init,
                        qn: !q_init,
                    },
                },
            };

            latch.set(s, e, r);

            assert_eq!(
                latch.q(),
                q_expected,
                "failed for inputs: {:?}",
                (q_init, s, r)
            )
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
            let mut latch = DLatch {
                sr_latch: SRLatchActiveHigh {
                    sr_latch_active_low: SRLatchActiveLow {
                        q: q_init,
                        qn: !q_init,
                    },
                },
            };

            latch.set(e, d);

            assert_eq!(
                latch.q(),
                q_expected,
                "failed for inputs: {:?}",
                (q_init, e, d)
            );

            assert_eq!(
                latch.qn(),
                !q_expected,
                "failed for inputs: {:?}",
                (q_init, e, d)
            )
        }
    }
}
