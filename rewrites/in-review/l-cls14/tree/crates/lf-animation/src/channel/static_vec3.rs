//! A constant vector channel: one padded value for every frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelStaticVector3`. The
//! 32-bit object points at a 16-byte value block (x, y, z plus a padding
//! word); here the value is a [`Vec4`](lf_math::Vec4), the padding
//! preserved bit-exact because the copy reports it.

use lf_math::Vec4;

/// A vector that does not vary over the clip.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StaticVec3 {
    /// The constant value (`w` is padding, preserved bit-exact).
    value: Vec4,
}

impl StaticVec3 {
    /// A constant channel holding `value`.
    #[must_use]
    pub const fn new(value: Vec4) -> Self {
        Self { value }
    }

    /// The constant value, padding included.
    ///
    /// The original copies the 16 bytes out and answers the padding word;
    /// the lift returns the whole value, which carries that word already.
    #[must_use]
    pub const fn get(self) -> Vec4 {
        self.value
    }

    /// Copies `records[0]` into the value, then checks that every one of
    /// `count` records matches it: per record the largest squared
    /// componentwise distance (x, y, z; the padding plays no part) must
    /// not exceed `tol * tol`, compared with the original's ordered-maximum
    /// chain and unordered-means-pass comparison. Returns true when every
    /// record passes (or `count <= 1`).
    ///
    /// # Panics
    ///
    /// When `records` is empty (the original always copies the first
    /// record) or holds fewer than `count` records with `count > 1`.
    pub fn adopt_if_uniform(&mut self, records: &[Vec4], count: i32, tol: f32) -> bool {
        let first = records[0];
        self.value = first;
        if count <= 1 {
            return true;
        }
        let limit = tol * tol;
        let mut k: i32 = 1;
        while k < count {
            let r = records[k as usize];
            let dx = first.x - r.x;
            let dy = first.y - r.y;
            let dz = first.z - r.z;
            // Ordered-maximum chain, exactly as verified: each comparison
            // replaces on not-greater, so NaN falls through to the next
            // candidate rather than winning.
            let mut m = dx * dx;
            let my = dy * dy;
            if !(m > my) {
                m = my;
            }
            let mz = dz * dz;
            if !(m > mz) {
                m = mz;
            }
            if m > limit {
                return false;
            }
            k += 1;
        }
        true
    }
}
