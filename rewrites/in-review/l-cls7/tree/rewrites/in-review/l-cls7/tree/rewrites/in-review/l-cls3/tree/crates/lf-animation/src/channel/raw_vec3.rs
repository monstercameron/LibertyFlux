//! An uncompressed vector channel: one padded key per frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelRawVector3`. The
//! 32-bit object points at 16-byte keys (x, y, z plus a padding word)
//! with a 16-bit count; here the keys are a vector in frame order, the
//! padding carried along bit-exact because the snap path copies and
//! reports it.

use lf_math::{Vec3, Vec4};

use super::frame::AnimChannel;
use super::frame::{SNAP_HI, SNAP_LO, clamp_index, key_below};

/// Matches the 32-bit header: the key count is a 16-bit word.
const MAX_KEYS: usize = 0xFFFF;

/// One padded vector per frame, interpolated between keys.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RawVec3 {
    /// The keys in frame order (`w` is padding, preserved bit-exact).
    keys: Vec<Vec4>,
}

impl RawVec3 {
    /// A channel over `keys` (frame `k` decodes from `keys[k]`).
    ///
    /// # Panics
    ///
    /// When `keys` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(keys: Vec<Vec4>) -> Self {
        assert!(keys.len() <= MAX_KEYS, "key count exceeds 16 bits");
        Self { keys }
    }

    /// The keys in frame order.
    #[must_use]
    pub fn keys(&self) -> &[Vec4] {
        &self.keys
    }

    /// Blends keys `idx` and `idx + 1` at fraction `t`, per component:
    /// the difference times `t`, plus the lower key, in that order (the
    /// addition takes the product first, which the not-a-number payload
    /// can observe). The padding words play no part.
    ///
    /// # Panics
    ///
    /// When `idx + 1` is past the last key.
    #[must_use]
    pub fn lerp_at(&self, idx: u32, t: f32) -> Vec3 {
        let lo = self.keys[idx as usize];
        let hi = self.keys[(idx as usize).wrapping_add(1)];
        Vec3 {
            x: (hi.x - lo.x) * t + lo.x,
            y: (hi.y - lo.y) * t + lo.y,
            z: (hi.z - lo.z) * t + lo.z,
        }
    }

    /// Byte size of the 32-bit storage form: one 16-byte slot per key
    /// plus one extra slot past the last key (the sampler reads element
    /// `index + 1`), wrapping exactly like the original.
    #[must_use]
    pub fn storage_size(&self) -> u32 {
        (self.keys.len() as u32).wrapping_add(1).wrapping_shl(4)
    }
}

impl AnimChannel for RawVec3 {
    type Sample = Vec4;

    /// Decodes the value at frame `frame` into `out`: on the snap path
    /// the whole key including padding, on the lerp path x, y, z only
    /// (the padding word is left untouched, as the original leaves the
    /// output's fourth word alone there). Out-of-range frames clamp to
    /// the end keys.
    ///
    /// The original's integer answer (the output address on the lerp path,
    /// the key's padding word on the snap path) is not modelled: the
    /// padding is already in `out` on the snap path.
    ///
    /// # Panics
    ///
    /// When the channel holds no keys: the original clamps to key -1
    /// there, which has no meaning here.
    fn sample_into(&self, frame: f32, out: &mut Vec4) {
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
                out.x = (b.x - a.x) * frac + a.x;
                out.y = (b.y - a.y) * frac + a.y;
                out.z = (b.z - a.z) * frac + a.z;
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
