//! A constant float channel: one value for every frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelStaticFloat`. The
//! 32-bit object is a 12-byte header holding the value inline; here the
//! value is just an `f32`.

use super::frame::AnimChannel;

/// A float that does not vary over the clip.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StaticFloat {
    /// The constant value.
    value: f32,
}

impl StaticFloat {
    /// A constant channel holding `value`.
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    /// The constant value.
    #[must_use]
    pub const fn get(self) -> f32 {
        self.value
    }

    /// Evaluates the channel, ignoring index, time and flags: the value.
    ///
    /// The original returns it in floating point; the lift returns it as
    /// an `f32`, the same bits.
    #[must_use]
    pub const fn eval(self) -> f32 {
        self.value
    }

    /// Copies the value out, ignoring the key index and blend: the value.
    #[must_use]
    pub const fn copy_key(self) -> f32 {
        self.value
    }

    /// Adopts `samples[0]` as the value, then checks that every one of
    /// `count` strided samples lies within `tol` of it, comparing squared
    /// differences against the squared tolerance in the original's order.
    /// Returns true when all match (or `count <= 1`).
    ///
    /// Element `k` is `samples[k * (stride + 1)]`: the original steps
    /// `stride * 4 + 4` bytes through floats.
    ///
    /// # Panics
    ///
    /// When `samples` is empty (the original always reads the first
    /// sample) or holds fewer than the `count` strided elements (the
    /// original would read past its input there too).
    pub fn adopt_if_uniform(
        &mut self,
        samples: &[f32],
        count: i32,
        stride: u32,
        tol: f32,
    ) -> bool {
        let first = samples[0];
        self.value = first;
        if count <= 1 {
            return true;
        }
        let step = stride.wrapping_add(1) as usize;
        let limit = tol * tol;
        let mut k: i32 = 1;
        while k < count {
            let x = samples[(k as usize).wrapping_mul(step)];
            let diff = first - x;
            if diff * diff > limit {
                return false;
            }
            k += 1;
        }
        true
    }
}

impl AnimChannel for StaticFloat {
    type Sample = f32;

    /// The value, whatever the frame.
    fn sample_into(&self, _frame: f32, out: &mut f32) {
        *out = self.value;
    }

    /// A constant channel holds one key.
    fn key_count(&self) -> usize {
        1
    }
}
