//! A constant quaternion channel: one rotation for every frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelStaticQuaternion`.
//! The 32-bit object points at a 16-byte value block; here the value is a
//! [`Quat`](lf_math::Quat). This is the channel behind the format
//! reader's constant quaternion: `lf-anim` decodes the four floats and
//! [`StaticQuat::new`] takes them.

use lf_math::Quat;

use super::frame::AnimChannel;

/// A rotation that does not vary over the clip.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StaticQuat {
    /// The constant rotation.
    value: Quat,
}

impl StaticQuat {
    /// A constant channel holding `value`.
    #[must_use]
    pub const fn new(value: Quat) -> Self {
        Self { value }
    }

    /// The constant rotation.
    #[must_use]
    pub const fn get(self) -> Quat {
        self.value
    }

    /// Copies `quats[0]` into the value, then checks that every one of
    /// `count` quaternions matches it: per record the largest squared
    /// componentwise distance (all four components) must not exceed
    /// `tol * tol`, through the original's pairwise-maximum ladder
    /// (`z`-vs-`w`, `x`-vs-`y`, then the winners). Returns true when every
    /// record passes (or `count <= 1`).
    ///
    /// # Panics
    ///
    /// When `quats` is empty (the original always copies the first
    /// record) or holds fewer than `count` records with `count > 1`.
    pub fn adopt_if_uniform(&mut self, quats: &[Quat], count: i32, tol: f32) -> bool {
        let first = quats[0];
        self.value = first;
        if count <= 1 {
            return true;
        }
        let limit = tol * tol;
        let mut k: i32 = 1;
        while k < count {
            let r = quats[k as usize];
            let dx = first.x - r.x;
            let dy = first.y - r.y;
            let dz = first.z - r.z;
            let dw = first.w - r.w;
            let dx2 = dx * dx;
            let dy2 = dy * dy;
            let dz2 = dz * dz;
            let dw2 = dw * dw;
            // Pairwise-maximum ladder, exactly as verified: each step keeps
            // the greater side, so NaN falls through to the other candidate.
            let m1 = if dz2 > dw2 { dz2 } else { dw2 };
            let m0 = if dx2 > dy2 { dx2 } else { dy2 };
            let m = if m0 > m1 { m0 } else { m1 };
            if m > limit {
                return false;
            }
            k += 1;
        }
        true
    }
}

impl AnimChannel for StaticQuat {
    type Sample = Quat;

    /// The rotation, whatever the frame.
    fn sample_into(&self, _frame: f32, out: &mut Quat) {
        *out = self.value;
    }

    /// A constant channel holds one key.
    fn key_count(&self) -> usize {
        1
    }
}
