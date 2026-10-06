//! An uncompressed quaternion channel: one key per frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelRawQuaternion`.
//! Only the indexed blend is lifted: the frame sampler runs through the
//! normalize callee and the builder through the allocator, neither
//! modelled yet. The 32-bit object points at 16-byte keys (x, y, z, w)
//! with a 16-bit count; here the keys are a vector in frame order.

use lf_math::Quat;

use super::frame::ONE;

/// Matches the 32-bit header: the key count is a 16-bit word.
const MAX_KEYS: usize = 0xFFFF;

/// One quaternion per frame, blended between keys.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RawQuat {
    /// The keys in frame order.
    keys: Vec<Quat>,
}

impl RawQuat {
    /// A channel over `keys` (frame `k` decodes from `keys[k]`).
    ///
    /// # Panics
    ///
    /// When `keys` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(keys: Vec<Quat>) -> Self {
        assert!(keys.len() <= MAX_KEYS, "key count exceeds 16 bits");
        Self { keys }
    }

    /// The keys in frame order.
    #[must_use]
    pub fn keys(&self) -> &[Quat] {
        &self.keys
    }

    /// Blends keys `idx` and `idx + 1` at fraction `t`, per component
    /// `lo * (1 - t) + hi * t` in that order, then normalizes: the
    /// squared length folds left (`((x*x + y*y) + z*z) + w*w`), and
    /// unless it is zero every component is multiplied by one over its
    /// square root (component first). A zero length skips the root and
    /// answers the blend as is; a not-a-number length takes the root
    /// path, as the original's zero test does.
    ///
    /// # Panics
    ///
    /// When `idx + 1` is past the last key.
    #[must_use]
    pub fn lerp_normalized(&self, idx: u32, t: f32) -> Quat {
        let s = ONE - t;
        let lo = self.keys[idx as usize];
        let hi = self.keys[(idx as usize).wrapping_add(1)];
        let x = lo.x * s + hi.x * t;
        let y = lo.y * s + hi.y * t;
        let z = lo.z * s + hi.z * t;
        let w = lo.w * s + hi.w * t;
        let n2 = (x * x + y * y) + z * z + w * w;
        if n2 == 0.0 {
            return Quat { x, y, z, w };
        }
        let inv = ONE / n2.sqrt();
        Quat {
            x: x * inv,
            y: y * inv,
            z: z * inv,
            w: w * inv,
        }
    }
}
