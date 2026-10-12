//! A run-length-coded integer channel: sample values plus a bit stream.
//!
//! Only the allocation size of `crAnimChannelRleInt` is lifted: the
//! decoders run through the bit-decoder callee and the compressor through
//! array and allocator callees, none modelled yet, so this type holds
//! only the sample count and the bit length the size reads.

/// Bytes of 32-bit header ahead of the samples and bit words.
const HEADER_SIZE: u32 = 0x1C;

/// Sample count and bit length of a run-length channel; the samples, the
/// bit stream and the shift are not modelled yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RleInt {
    /// Number of samples.
    sample_count: u16,
    /// Length of the bit stream in bits.
    bit_len: u32,
}

impl RleInt {
    /// A channel fragment with `sample_count` samples and `bit_len` bits.
    #[must_use]
    pub const fn new(sample_count: u16, bit_len: u32) -> Self {
        Self {
            sample_count,
            bit_len,
        }
    }

    /// Number of samples.
    #[must_use]
    pub const fn sample_count(self) -> u16 {
        self.sample_count
    }

    /// Length of the bit stream in bits.
    #[must_use]
    pub const fn bit_len(self) -> u32 {
        self.bit_len
    }

    /// Byte size of the 32-bit storage form: the bit length rounded up to
    /// whole 32-bit words, plus one word per sample, times four, plus the
    /// header, wrapping exactly like the original.
    #[must_use]
    pub const fn alloc_size(self) -> u32 {
        // The rounding add cannot overflow (at most 2^27 + 1), as verified.
        let words = (self.bit_len >> 5) + ((self.bit_len & 0x1F) != 0) as u32;
        let total = words.wrapping_add(self.sample_count as u32);
        total.wrapping_mul(4).wrapping_add(HEADER_SIZE)
    }
}
