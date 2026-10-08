//! Typed, pointer-free forwarding for a callback that receives two integer
//! words followed by one single-precision value.
//!
//! The wrapper represented here supplies `(0, 0, value)` and returns the
//! callback's `u32` answer unchanged. [`FloatCallback`] is an injected
//! collaborator; this module does not bind or describe the native callback's
//! implementation or effects.

use lf_core::boundary::FixedLayout;

/// One IEEE-754 binary32 word, compared and transported without numeric
/// equality or arithmetic.
///
/// Unlike `lf-lift-diff::F32`, equality here includes the sign and payload of
/// every NaN because those bits are callback arguments, not computed results.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FloatBits(u32);

impl FloatBits {
    /// Carries the supplied IEEE-754 word without interpreting it.
    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// The exact word carried by this value.
    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Captures an `f32` representation without performing arithmetic.
    #[must_use]
    pub const fn from_f32(value: f32) -> Self {
        Self(value.to_bits())
    }

    /// Reconstitutes the same IEEE-754 word as an `f32` value.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        f32::from_bits(self.0)
    }
}

impl FixedLayout for FloatBits {
    const SIZE: usize = 4;

    fn decode(bytes: &[u8]) -> Self {
        Self(u32::decode(bytes))
    }

    fn encode(&self, out: &mut [u8]) {
        self.0.encode(out);
    }
}

lf_core::assert_fixed_layout!(FloatBits);

/// The callback's ordered positional arguments: two `u32` words, then the
/// exact binary32 payload word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatCallbackArgs {
    /// First integer callback argument.
    pub arg0: u32,
    /// Second integer callback argument.
    pub arg1: u32,
    /// Third callback argument, retained as its raw IEEE-754 bits.
    pub value: FloatBits,
}

impl FloatCallbackArgs {
    /// Returns the three callback words in ABI argument order.
    #[must_use]
    pub const fn words(self) -> [u32; 3] {
        [self.arg0, self.arg1, self.value.bits()]
    }
}

/// A callback collaborator supplied by the caller of the portable lift.
pub trait FloatCallback {
    /// Receives ordered arguments and returns the callback's `u32` answer.
    fn call(&mut self, args: FloatCallbackArgs) -> u32;
}

/// Forwards `(0, 0, value)` and returns the collaborator's answer unchanged.
#[must_use]
pub fn forward_zero_flags<C: FloatCallback + ?Sized>(
    callback: &mut C,
    value: FloatBits,
) -> u32 {
    callback.call(FloatCallbackArgs {
        arg0: 0,
        arg1: 0,
        value,
    })
}
