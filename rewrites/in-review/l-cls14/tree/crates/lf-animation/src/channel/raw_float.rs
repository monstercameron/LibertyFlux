//! An uncompressed float channel: one key per frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelRawFloat`. The
//! 32-bit object points at its key array with a 16-bit count; here the
//! keys are a vector, one float per frame in frame order.

use super::frame::AnimChannel;
use super::frame::{SNAP_HI, SNAP_LO, clamp_index, key_below};

/// Matches the 32-bit header: the key count is a 16-bit word.
const MAX_KEYS: usize = 0xFFFF;

/// One float per frame, interpolated between keys.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RawFloat {
    /// The keys in frame order.
    keys: Vec<f32>,
}

impl RawFloat {
    /// A channel over `keys` (frame `k` decodes from `keys[k]`).
    ///
    /// # Panics
    ///
    /// When `keys` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(keys: Vec<f32>) -> Self {
        assert!(keys.len() <= MAX_KEYS, "key count exceeds 16 bits");
        Self { keys }
    }

    /// The keys in frame order.
    #[must_use]
    pub fn keys(&self) -> &[f32] {
        &self.keys
    }

    /// Blends keys `idx` and `idx + 1` at fraction `t`: the difference
    /// times `t`, plus the lower key, in that order.
    ///
    /// # Panics
    ///
    /// When `idx + 1` is past the last key.
    #[must_use]
    pub fn lerp_at(&self, idx: u32, t: f32) -> f32 {
        let a = self.keys[idx as usize];
        let b = self.keys[(idx as usize).wrapping_add(1)];
        (b - a) * t + a
    }

    /// The same blend, widened to `f64` exactly (the original returns it
    /// in floating point at double width).
    ///
    /// # Panics
    ///
    /// When `idx + 1` is past the last key.
    #[must_use]
    pub fn lerp_at_f64(&self, idx: u32, t: f32) -> f64 {
        f64::from(self.lerp_at(idx, t))
    }
}

impl AnimChannel for RawFloat {
    type Sample = f32;

    /// Decodes the value at frame `frame`: the key at or below the frame
    /// when the fraction snaps to it, else the blend of the two bracketing
    /// keys. Out-of-range frames clamp to the end keys.
    ///
    /// # Panics
    ///
    /// When the channel holds no keys: the original clamps to key -1
    /// there, which has no meaning here.
    fn sample_into(&self, frame: f32, out: &mut f32) {
        assert!(!self.keys.is_empty(), "sampling an empty channel");
        let (index, frac) = key_below(frame);
        let last = self.keys.len() as i32 - 1;
        if !(frac > SNAP_HI) {
            if !(frac > SNAP_LO) {
                *out = self.keys[clamp_index(index, last) as usize];
            } else {
                let j = clamp_index(index.wrapping_add(1), last);
                let i = clamp_index(index, last);
                let a = self.keys[i as usize];
                let b = self.keys[j as usize];
                *out = (b - a) * frac + a;
            }
        } else {
            let k = clamp_index(index.wrapping_add(1), last);
            *out = self.keys[k as usize];
        }
    }

    fn key_count(&self) -> usize {
        self.keys.len()
    }
}
