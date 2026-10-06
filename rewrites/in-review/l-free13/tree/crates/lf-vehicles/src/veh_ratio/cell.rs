//! One derived tuning coefficient: two source floats and their quotient.

/// A numerator/denominator pair with the quotient stored beside them.
///
/// This is the data every member of the ratio family shares: the
/// original's three globals (two adjacent source words and one
/// destination word) as three owned `f32` fields. A cell is built from
/// the sources' current values; [`RatioCell::refresh`] recomputes the
/// quotient exactly as the original does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RatioCell {
    /// The dividend: the original's numerator global.
    numer: f32,
    /// The divisor: the original's denominator global.
    denom: f32,
    /// The stored quotient: the original's destination global.
    value: f32,
}

impl RatioCell {
    /// Builds a cell from its two sources and its stored quotient.
    #[must_use]
    pub fn new(numer: f32, denom: f32, value: f32) -> Self {
        Self {
            numer,
            denom,
            value,
        }
    }

    /// The dividend.
    #[must_use]
    pub fn numer(&self) -> f32 {
        self.numer
    }

    /// The divisor.
    #[must_use]
    pub fn denom(&self) -> f32 {
        self.denom
    }

    /// The last stored quotient.
    #[must_use]
    pub fn value(&self) -> f32 {
        self.value
    }

    /// Replaces both sources, leaving the stored quotient untouched.
    pub fn set_sources(&mut self, numer: f32, denom: f32) {
        self.numer = numer;
        self.denom = denom;
    }

    /// Recomputes the quotient as `numer / denom` in single precision.
    ///
    /// This is the whole behaviour of every verified member of the
    /// ratio family: one IEEE-754 single-precision division in the
    /// original's operand order (pinned so the compiler keeps it), with
    /// the quotient stored back. A zero denominator yields a signed
    /// infinity, zero over zero a quiet NaN, and infinities and NaNs
    /// propagate as the hardware defines; the division never faults.
    pub fn refresh(&mut self) {
        self.value =
            core::hint::black_box(self.numer) / core::hint::black_box(self.denom);
    }
}
