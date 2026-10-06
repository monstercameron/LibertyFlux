//! A delta-coded float channel: residuals packed into bit streams.
//!
//! Only the storage size of `crAnimChannelDeltaFloat` is lifted: the
//! sampler and the compressor run through sub-object and allocator
//! callees that are not modelled yet, so this type holds only the sample
//! count the size reads.

/// Bytes of 32-bit header ahead of the packed words.
const HEADER_SIZE: u32 = 0x38;

/// Sample count of a delta-float channel; the streams are not modelled yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DeltaFloat {
    /// Number of samples.
    count: u32,
}

impl DeltaFloat {
    /// A channel fragment holding `count` samples.
    #[must_use]
    pub const fn new(count: u32) -> Self {
        Self { count }
    }

    /// Number of samples.
    #[must_use]
    pub const fn count(self) -> u32 {
        self.count
    }

    /// Byte size of the 32-bit storage form: the sample count rounded up
    /// to whole 32-bit words, times four, plus the header, wrapping
    /// exactly like the original.
    #[must_use]
    pub const fn storage_size(self) -> u32 {
        let words = (self.count >> 5).wrapping_add(((self.count & 31) != 0) as u32);
        words.wrapping_mul(4).wrapping_add(HEADER_SIZE)
    }
}
