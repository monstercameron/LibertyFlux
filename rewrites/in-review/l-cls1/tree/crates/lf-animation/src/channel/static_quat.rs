//! A constant quaternion channel: one rotation for every frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelStaticQuaternion`.
//! The 32-bit object points at a 16-byte value block; here the value is a
//! [`Quat`](lf_math::Quat). This is the channel behind the format
//! reader's constant quaternion: `lf-anim` decodes the four floats and
//! [`StaticQuat::new`] takes them.
//!
//! Only the copy is verified, so only it is lifted: there is no verified
//! sampler, size or uniformity check yet.

use lf_math::Quat;

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
}
