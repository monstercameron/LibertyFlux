//! A constant integer channel: one word for every frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelStaticInt`. Only the
//! uniformity check is verified, so only it is lifted: there is no
//! verified sampler or size yet.

/// An integer that does not vary over the clip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StaticInt {
    /// The constant word.
    value: u32,
}

impl StaticInt {
    /// A constant channel holding `value`.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    /// The constant word.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.value
    }

    /// Checks that the first `count` samples all equal their predecessor.
    /// On success stores `samples[0]` as the value and returns true; on
    /// the first mismatch returns false without storing.
    ///
    /// # Panics
    ///
    /// When `samples` is empty (the original always reads the first
    /// sample) or holds fewer than `count` elements with `count > 1`.
    pub fn adopt_if_uniform(&mut self, samples: &[u32], count: i32) -> bool {
        if count > 1 {
            let mut k: i32 = 1;
            while k < count {
                let prev = samples[(k as usize).wrapping_sub(1)];
                let cur = samples[k as usize];
                if prev != cur {
                    return false;
                }
                k += 1;
            }
        }
        self.value = samples[0];
        true
    }
}
