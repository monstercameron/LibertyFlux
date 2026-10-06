//! An uncompressed integer channel: one key per frame.
//!
//! Lifted from the verified rewrites of `crAnimChannelRawInt`. The 32-bit
//! object points at its key array with a 16-bit count; here the keys are
//! a vector, one word per frame in frame order.

use super::frame::AnimChannel;
use super::frame::{clamp_index, round_half_up};

/// Matches the 32-bit header: the key count is a 16-bit word.
const MAX_KEYS: usize = 0xFFFF;

/// Bytes of 32-bit header ahead of the keys.
const HEADER_SIZE: u32 = 0x10;

/// One word per frame, no blending.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawInt {
    /// The keys in frame order.
    keys: Vec<u32>,
}

impl RawInt {
    /// A channel over `keys` (frame `k` decodes from `keys[k]`).
    ///
    /// # Panics
    ///
    /// When `keys` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(keys: Vec<u32>) -> Self {
        assert!(keys.len() <= MAX_KEYS, "key count exceeds 16 bits");
        Self { keys }
    }

    /// The keys in frame order.
    #[must_use]
    pub fn keys(&self) -> &[u32] {
        &self.keys
    }

    /// Copies key `idx` out.
    ///
    /// # Panics
    ///
    /// When `idx` is past the last key.
    #[must_use]
    pub fn key_at(&self, idx: u32) -> u32 {
        self.keys[idx as usize]
    }

    /// Byte size of the 32-bit storage form: four bytes per key plus the
    /// 16-byte header, wrapping exactly like the original.
    #[must_use]
    pub fn storage_size(&self) -> u32 {
        (self.keys.len() as u32)
            .wrapping_mul(4)
            .wrapping_add(HEADER_SIZE)
    }
}

impl AnimChannel for RawInt {
    type Sample = u32;

    /// Decodes the value at frame `frame`: the frame rounded half up,
    /// clamped to the key range. Out-of-range frames clamp to the end keys.
    ///
    /// # Panics
    ///
    /// When the channel holds no keys: the original clamps to key -1
    /// there, which has no meaning here.
    fn sample_into(&self, frame: f32, out: &mut u32) {
        assert!(!self.keys.is_empty(), "sampling an empty channel");
        let last = self.keys.len() as i32 - 1;
        let i = clamp_index(round_half_up(frame), last);
        *out = self.keys[i as usize];
    }

    fn key_count(&self) -> usize {
        self.keys.len()
    }
}
