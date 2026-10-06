//! The pool vector: a count-prefixed fixed-stride element buffer.
//!
//! Lifted from the thirteen verified `pool_vec_init` instances: one
//! routine proved thirteen times, over each instance's stride and stamp.
//! The 32-bit form keeps a 12-byte header (element count, a spare word,
//! the buffer body) and a buffer holding the count plus `count` elements
//! of `stride` bytes, each stamped with its element-table word. The lift
//! owns the buffer as a byte vector; the header travels as its count.

use lf_core::boundary::Handle32;

/// Identity of a pool-vector element table: the stamp word's meaning.
///
/// Element tables live in code not yet lifted, so stamps travel as opaque
/// handles until their owner lifts.
#[derive(Debug)]
pub struct VecElemTag;

/// An opaque element stamp: the word each slot head carries.
pub type ElemStamp = Handle32<VecElemTag>;

/// Allocates a pool-vector buffer: the initialiser's allocator.
pub trait VecAlloc {
    /// Allocates `size` bytes of backing, or `None` on failure. The
    /// backing must be exactly `size` bytes long; the initialiser writes
    /// the count prefix and the stamps, leaving the rest as it came.
    fn alloc(&mut self, size: u32) -> Option<Vec<u8>>;
}

impl<F: FnMut(u32) -> Option<Vec<u8>>> VecAlloc for F {
    fn alloc(&mut self, size: u32) -> Option<Vec<u8>> {
        self(size)
    }
}

/// A fixed-stride element buffer: the count plus stamped slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolVec {
    /// Element stride in bytes.
    stride: u32,
    /// The buffer: the count word, then one stride per slot, each slot
    /// head carrying the stamp. Bytes past the stamps are backing as the
    /// allocator returned it (zeros from the constructors here; allocator
    /// garbage in the original).
    buf: Vec<u8>,
}

impl PoolVec {
    /// Allocation size for `count` elements of `stride` bytes:
    /// `count * stride + 4`, saturated to `u32::MAX` exactly as the
    /// 32-bit form saturates (on multiply overflow or add carry).
    #[must_use]
    pub const fn alloc_size(count: u32, stride: u32) -> u32 {
        let prod = (count as u64) * (stride as u64);
        if prod > 0xFFFF_FFFF {
            0xFFFF_FFFF
        } else {
            let base = prod as u32;
            match base.checked_add(4) {
                Some(size) => size,
                None => 0xFFFF_FFFF,
            }
        }
    }

    /// Allocates and initialises the element buffer for `count` slots.
    ///
    /// Returns `None` when the allocator fails (the 32-bit form clears
    /// the header body and returns 0). Otherwise every slot head carries
    /// `stamp` and the count word heads the buffer.
    ///
    /// # Panics
    ///
    /// When the backing is not exactly the computed size, or when the
    /// stride is below 4 with more than one slot (the 32-bit form writes
    /// the stamps past its buffer there).
    pub fn init(
        count: u32,
        stride: u32,
        stamp: ElemStamp,
        alloc: &mut impl VecAlloc,
    ) -> Option<Self> {
        let size = Self::alloc_size(count, stride);
        let mut buf = alloc.alloc(size)?;
        assert!(
            buf.len() as u64 == u64::from(size),
            "backing holds {} bytes for a {size}-byte buffer",
            buf.len()
        );
        buf[0..4].copy_from_slice(&count.to_le_bytes());
        let stamp = stamp.get().to_le_bytes();
        let mut slot = 4usize;
        let mut left = count;
        while left != 0 {
            buf[slot..slot + 4].copy_from_slice(&stamp);
            slot += stride as usize;
            left -= 1;
        }
        Some(Self { stride, buf })
    }

    /// Element stride in bytes.
    #[must_use]
    pub fn stride(&self) -> u32 {
        self.stride
    }

    /// The buffer: the count word, then the slots.
    #[must_use]
    pub fn buf(&self) -> &[u8] {
        &self.buf
    }

    /// Slot count, from the buffer's count word.
    #[must_use]
    pub fn count(&self) -> u32 {
        u32::from_le_bytes(self.buf[0..4].try_into().unwrap())
    }

    /// Offset one past the last slot: the buffer length. The 32-bit form
    /// answers this offset as an address (the body itself when empty).
    #[must_use]
    pub fn end_offset(&self) -> usize {
        self.buf.len()
    }
}
