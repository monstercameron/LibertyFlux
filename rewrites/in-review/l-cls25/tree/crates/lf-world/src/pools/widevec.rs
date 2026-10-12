//! The wide pool vector: a 16-byte header over stamped fixed-stride slots.
//!
//! Lifted from the seven verified wide initialiser instances: five share
//! the plain wide body (proved five times), one writes two extra words
//! per element and answers a bumped end, and one constructs each element
//! through a callee. The 32-bit form keeps a 16-byte header (element
//! count, a spare word, the element base) and a buffer holding the count
//! plus `count` elements of `stride` bytes. The lift owns the buffer as a
//! byte vector; the header travels as its count.

use crate::pools::{ElemBuild, ElemStamp, VecAlloc};

/// Bytes of header prefix inside the buffer (the count word plus padding).
pub const PREFIX: usize = 16;
/// Offset of the zero status word inside a plain wide element.
pub const STATUS_OFF: usize = 8;
/// Offset of the extra zero word in the wide-extra element.
pub const EXTRA_ZERO_OFF: usize = 0x60;
/// Offset of the trailing all-ones word in the wide-extra element.
pub const EXTRA_ONES_OFF: usize = 0x64;
/// The wide-extra all-ones word.
pub const ONES: u32 = 0xffff_ffff;

/// A wide element buffer: the count prefix plus stamped slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WideVec {
    /// Element stride in bytes.
    stride: u32,
    /// The buffer: the count word, twelve pad bytes, then one stride per
    /// slot, each slot head carrying the stamp. Bytes past the modelled
    /// words are backing as the allocator returned it.
    buf: Vec<u8>,
}

impl WideVec {
    /// Allocation size for `count` elements of `stride` bytes:
    /// `count * stride + 16`, saturated to `u32::MAX` exactly as the
    /// 32-bit form saturates.
    #[must_use]
    // The cast is exact: the arm above excluded every product over u32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    pub const fn alloc_size(count: u32, stride: u32) -> u32 {
        let prod = (count as u64) * (stride as u64) + 0x10;
        if prod > 0xFFFF_FFFF {
            0xFFFF_FFFF
        } else {
            prod as u32
        }
    }

    /// Allocates and initialises the element buffer for `count` slots:
    /// every slot head carries `stamp` and every status word is zero.
    /// Returns `None` when the allocator fails (the 32-bit form clears
    /// the header body and returns 0).
    ///
    /// # Panics
    ///
    /// When the backing is not exactly the computed size, or when the
    /// stride is below 12 with a non-empty pool (the 32-bit form writes
    /// the status words past its buffer there).
    pub fn init(
        count: u32,
        stride: u32,
        stamp: ElemStamp,
        alloc: &mut impl VecAlloc,
    ) -> Option<Self> {
        assert!(
            count == 0 || stride >= 12,
            "stride {stride} below 12 with {count} slots: the original writes past its buffer"
        );
        let size = Self::alloc_size(count, stride);
        let mut buf = alloc.alloc(size)?;
        assert!(
            buf.len() as u64 == u64::from(size),
            "backing holds {} bytes for a {size}-byte buffer",
            buf.len()
        );
        buf[0..4].copy_from_slice(&count.to_le_bytes());
        let stamp = stamp.get().to_le_bytes();
        let zero = 0u32.to_le_bytes();
        let mut slot = PREFIX;
        let mut left = count;
        while left != 0 {
            buf[slot..slot + 4].copy_from_slice(&stamp);
            buf[slot + STATUS_OFF..slot + STATUS_OFF + 4].copy_from_slice(&zero);
            slot += stride as usize;
            left -= 1;
        }
        Some(Self { stride, buf })
    }

    /// Allocates and initialises the wide-extra buffer: like [`WideVec::init`]
    /// plus a zero word at +0x60 and an all-ones word at +0x64 per element.
    ///
    /// # Panics
    ///
    /// When the backing is not exactly the computed size, or when the
    /// stride is below `0x68` with a non-empty pool.
    pub fn init_extra(
        count: u32,
        stride: u32,
        stamp: ElemStamp,
        alloc: &mut impl VecAlloc,
    ) -> Option<Self> {
        assert!(
            count == 0 || stride >= 0x68,
            "stride {stride:#x} below 0x68 with {count} slots: the original writes past its buffer"
        );
        let mut vec = Self::init(count, stride, stamp, alloc)?;
        let zero = 0u32.to_le_bytes();
        let ones = ONES.to_le_bytes();
        let mut slot = PREFIX;
        let mut left = count;
        while left != 0 {
            vec.buf[slot + EXTRA_ZERO_OFF..slot + EXTRA_ZERO_OFF + 4].copy_from_slice(&zero);
            vec.buf[slot + EXTRA_ONES_OFF..slot + EXTRA_ONES_OFF + 4].copy_from_slice(&ones);
            slot += stride as usize;
            left -= 1;
        }
        Some(vec)
    }

    /// Allocates the buffer and constructs each element through `build`,
    /// answering the last construction's answer (0, the buffer itself, when
    /// the pool is empty: the 32-bit form answers the block address).
    /// Returns `None` when the allocator fails.
    ///
    /// # Panics
    ///
    /// When the backing is not exactly the computed size.
    pub fn init_constructed(
        count: u32,
        stride: u32,
        alloc: &mut impl VecAlloc,
        build: &mut impl ElemBuild,
    ) -> Option<(Self, u32)> {
        let size = Self::alloc_size(count, stride);
        let mut buf = alloc.alloc(size)?;
        assert!(
            buf.len() as u64 == u64::from(size),
            "backing holds {} bytes for a {size}-byte buffer",
            buf.len()
        );
        buf[0..4].copy_from_slice(&count.to_le_bytes());
        let mut last = 0u32;
        let mut slot = PREFIX;
        let mut left = count;
        while left != 0 {
            let end = slot + stride as usize;
            assert!(
                end <= buf.len(),
                "slot {slot} of stride {stride} past the {size}-byte buffer"
            );
            last = build.build_elem(&mut buf[slot..end]);
            slot = end;
            left -= 1;
        }
        Some((Self { stride, buf }, last))
    }

    /// Element stride in bytes.
    #[must_use]
    pub fn stride(&self) -> u32 {
        self.stride
    }

    /// The buffer: the count word, the pad, then the slots.
    #[must_use]
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    /// Slot count, from the buffer's count word.
    ///
    /// # Panics
    ///
    /// When the buffer is shorter than the count word (only a corrupt
    /// `WideVec` can do that).
    #[must_use]
    pub fn count(&self) -> u32 {
        u32::from_le_bytes(self.buf[0..4].try_into().unwrap())
    }

    /// Offset one past the last slot: the buffer length. The plain wide
    /// form answers this offset as an address (the body itself when empty).
    #[must_use]
    pub fn end_offset(&self) -> usize {
        self.buf.len()
    }

    /// End offset of the wide-extra form: 8 past [`WideVec::end_offset`]
    /// (the original's bumped pointer), or 0 for an empty pool (the
    /// original answers the block itself there).
    #[must_use]
    pub fn end_offset_extra(&self) -> usize {
        if self.count() == 0 {
            0
        } else {
            self.buf.len() + 8
        }
    }
}
