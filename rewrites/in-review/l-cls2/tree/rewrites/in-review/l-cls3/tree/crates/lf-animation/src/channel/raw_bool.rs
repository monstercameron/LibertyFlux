//! An uncompressed boolean channel: one packed byte per eight frames.
//!
//! Lifted from the verified rewrites of `crAnimChannelRawBool`. The 32-bit
//! object points at a byte array with a 16-bit byte count; here the bytes
//! are a vector in frame order, eight frames per byte.
//!
//! The decode is unusual, restated exactly as verified: the frame rounds
//! half up to an index, byte `index >> 3` (clamped into range) is ANDed
//! with the low index byte, and the answer is whether any of the low three
//! bits survive.

use super::frame::AnimChannel;
use super::frame::round_half_up;

/// Matches the 32-bit header: the byte count is a 16-bit word.
const MAX_BYTES: usize = 0xFFFF;

/// Bytes of 32-bit header ahead of the bytes.
const HEADER_SIZE: u32 = 0x10;

/// Eight frames per byte, decoded without blending.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RawBool {
    /// The packed bytes in frame order.
    bytes: Vec<u8>,
}

impl RawBool {
    /// A channel over `bytes` (frames `8*k..8*k+8` decode from `bytes[k]`).
    ///
    /// # Panics
    ///
    /// When `bytes` holds more than 65,535 entries: the original's count
    /// is a 16-bit word.
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        assert!(bytes.len() <= MAX_BYTES, "byte count exceeds 16 bits");
        Self { bytes }
    }

    /// The packed bytes in frame order.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Byte size of the 32-bit storage form: one byte per entry plus the
    /// 16-byte header, wrapping exactly like the original.
    #[must_use]
    pub fn storage_size(&self) -> u32 {
        (self.bytes.len() as u32).wrapping_add(HEADER_SIZE)
    }
}

impl AnimChannel for RawBool {
    type Sample = u8;

    /// Decodes the value at frame `frame`: 1 when the masked byte keeps
    /// any of its low three bits, else 0.
    ///
    /// # Panics
    ///
    /// When the channel holds no bytes, or when the frame selects past
    /// the last byte: the original reads one byte past its buffer there
    /// (allocator slack), which has no meaning here.
    fn sample_into(&self, frame: f32, out: &mut u8) {
        assert!(!self.bytes.is_empty(), "sampling an empty channel");
        let raw = round_half_up(frame);
        let index = if raw < 0 { 0u32 } else { raw as u32 };
        let byte = index >> 3;
        let count = self.bytes.len() as u32;
        assert!(byte < count, "frame selects past the last packed byte");
        let masked = self.bytes[byte as usize] & (index as u8);
        *out = u8::from((masked & 7) != 0);
    }

    /// Number of packed bytes (eight frames each).
    fn key_count(&self) -> usize {
        self.bytes.len()
    }
}
