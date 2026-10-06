//! Voice parameter blocks and the voice header reset.
//!
//! Lifted from the verified rewrites of the `audio_voice_defaults` pair
//! (reset five 12-byte blocks to `(0, 1.0, 0)`, the second also reining in
//! a sub-object through a helper) and the `audio_voice_reset` header
//! (clear one word, tag the next half-word). The 32-bit routines answer
//! their own `this` address; the lift answers nothing and the proof pins
//! the echo per case.

/// How many parameter blocks one bank holds.
pub const BLOCK_COUNT: usize = 5;
/// Stride of one parameter block in bytes.
pub const BLOCK_STRIDE: usize = 12;

/// One voice parameter block: a code word, a gain float, a flag half-word.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParamBlock {
    /// The code word (block + 0).
    pub code: u32,
    /// The gain float (block + 4).
    pub gain: f32,
    /// The flag half-word (block + 8).
    pub flags: u16,
}

impl ParamBlock {
    /// The default block the reset routines write: `(0, 1.0, 0)`.
    pub const DEFAULT: Self = Self {
        code: 0,
        gain: 1.0,
        flags: 0,
    };
}

/// Five parameter blocks back to back.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamBlocks {
    /// The blocks in order.
    pub blocks: [ParamBlock; BLOCK_COUNT],
}

impl ParamBlocks {
    /// Blocks from five values.
    #[must_use]
    pub const fn new(blocks: [ParamBlock; BLOCK_COUNT]) -> Self {
        Self { blocks }
    }

    /// Reset every block to [`ParamBlock::DEFAULT`].
    pub fn reset(&mut self) {
        self.blocks = [ParamBlock::DEFAULT; BLOCK_COUNT];
    }

    /// Reset every block, then reinit the sub-object through its helper.
    pub fn reset_and_init_sub(&mut self, sub: &mut impl SubInit) {
        self.reset();
        sub.init_sub();
    }
}

/// Reinitialises the sub-object past the parameter blocks: the second
/// reset routine's helper call.
pub trait SubInit {
    /// Runs the helper.
    fn init_sub(&mut self);
}

impl<F: FnMut()> SubInit for F {
    fn init_sub(&mut self) {
        self();
    }
}

/// The two-word voice header: a sequence word and a tag half-word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoiceHead {
    /// The sequence word (object + 0).
    pub seq: u32,
    /// The tag half-word (object + 4).
    pub tag: u16,
}

/// Tag value the reset routine stamps into the header.
pub const RESET_TAG: u16 = 0xFFFF;

impl VoiceHead {
    /// Reset the header: clear the sequence word, stamp [`RESET_TAG`].
    pub fn reset(&mut self) {
        self.seq = 0;
        self.tag = RESET_TAG;
    }
}
