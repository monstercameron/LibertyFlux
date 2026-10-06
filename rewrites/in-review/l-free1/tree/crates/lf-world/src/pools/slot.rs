//! The slot pool: entries, flag bytes and stride.
//!
//! Lifted from the verified rewrites of the free pool routines (no class):
//! the context cluster (creation, occupancy, data word, assignment, the
//! indexed store), the seven cursor-step instances (one routine, proved
//! seven times) and the slot-validity check. The 32-bit descriptor is four
//! words (entry base, flag bytes, slot count, entry stride); the lift owns
//! the entries and the flags as byte vectors and the count is the flag
//! store's length.

use lf_core::boundary::Handle32;

/// Flag bit marking a dead slot. Every lookup tests this bit and no other.
pub const DEAD_BIT: u8 = 0x80;

/// Byte offset of the data word inside an entry.
const DATA_OFF: u64 = 0x04;
/// Byte offset of the successor index inside an entry.
const NEXT_OFF: u64 = 0x0c;
/// Successor value meaning "no next entry".
const NO_NEXT: u32 = 0xffff_ffff;
/// Size in bytes of a pool context block.
const CTX_SIZE: u32 = 0x1c;
/// Initialiser tag the creation routine passes to the context initialiser.
const INIT_TAG: u32 = 0x10;
/// Row stride of the indexed store's table, in words (100 bytes).
const TABLE_ROW_WORDS: u32 = 25;
/// Row-base offset of the indexed store's table, in words (`0x58` bytes).
const TABLE_BASE_WORDS: u32 = 0x16;

/// Identity of a pool context block.
///
/// The 28-byte context layout is not lifted yet (the initialiser that fills
/// it is not among the verified functions), so contexts travel as opaque
/// handles until it is.
#[derive(Debug)]
pub struct CtxTag;

/// An opaque pool context: the creation routine's answer.
pub type CtxHandle = Handle32<CtxTag>;

/// Allocates a pool context block: the creation routine's allocator.
pub trait CtxAlloc {
    /// Allocates `size` bytes, returning the block or `None` on failure.
    fn alloc(&mut self, size: u32) -> Option<CtxHandle>;
}

/// Fills a pool context block: the creation routine's initialiser.
pub trait CtxInit {
    /// Initialises `block` with the two creation arguments and the tag,
    /// returning the context to publish.
    fn init(&mut self, block: CtxHandle, a0: u32, a1: u32, tag: u32) -> CtxHandle;
}

/// Notifies that a slot's head word became non-zero: the assignment callee.
pub trait Notify {
    /// Records the assignment of slot `index`.
    fn notify(&mut self, index: u32);
}

/// Refreshes the pool before an indexed store, answering a table row.
pub trait Refresh {
    /// Runs the refresh, answering the table row index.
    ///
    /// The 32-bit callee answers a row base *address*; the lift translates
    /// it to the row index into the table the caller passes. The
    /// differential proof scripts rows and plants the translated addresses.
    fn refresh(&mut self) -> u32;
}

impl<F: FnMut(u32)> Notify for F {
    fn notify(&mut self, index: u32) {
        self(index);
    }
}

impl<F: FnMut() -> u32> Refresh for F {
    fn refresh(&mut self) -> u32 {
        self()
    }
}

impl<F: FnMut(u32) -> Option<CtxHandle>> CtxAlloc for F {
    fn alloc(&mut self, size: u32) -> Option<CtxHandle> {
        self(size)
    }
}

impl<F: FnMut(CtxHandle, u32, u32, u32) -> CtxHandle> CtxInit for F {
    fn init(&mut self, block: CtxHandle, a0: u32, a1: u32, tag: u32) -> CtxHandle {
        self(block, a0, a1, tag)
    }
}

/// A fixed-size slot store: `flags.len()` slots of `stride` bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotPool {
    /// The entries back to back, `flags.len() * stride` bytes.
    entries: Vec<u8>,
    /// One flag byte per slot; bit [`DEAD_BIT`] means dead.
    flags: Vec<u8>,
    /// Entry stride in bytes.
    stride: u32,
}

impl SlotPool {
    /// A pool over `entries` with `flags`, `stride` bytes per slot.
    ///
    /// # Panics
    ///
    /// When `entries` does not hold exactly one stride per flag byte.
    #[must_use]
    pub fn from_parts(entries: Vec<u8>, flags: Vec<u8>, stride: u32) -> Self {
        assert!(
            entries.len() as u64 == flags.len() as u64 * u64::from(stride),
            "entries hold {} bytes for {} slots of stride {stride}",
            entries.len(),
            flags.len()
        );
        Self {
            entries,
            flags,
            stride,
        }
    }

    /// Number of slots.
    #[must_use]
    pub fn slot_count(&self) -> usize {
        self.flags.len()
    }

    /// Entry stride in bytes.
    #[must_use]
    pub fn stride(&self) -> u32 {
        self.stride
    }

    /// The entries back to back.
    #[must_use]
    pub fn entries(&self) -> &[u8] {
        &self.entries
    }

    /// The per-slot flag bytes.
    #[must_use]
    pub fn flags(&self) -> &[u8] {
        &self.flags
    }

    /// Flag byte of slot `index`.
    ///
    /// # Panics
    ///
    /// When `index` is past the flag store (the original reads past it).
    fn flag(&self, index: u32) -> u8 {
        *self.flags.get(index as usize).unwrap_or_else(|| {
            panic!(
                "slot {index} past {} slots: the original reads past the flag store",
                self.flags.len()
            )
        })
    }

    /// Byte offset of slot `index`'s entry, checked for an access of `need`
    /// bytes from the entry start.
    ///
    /// # Panics
    ///
    /// When the access would leave the entry store (the original reads or
    /// writes the wrapped address regardless).
    fn slot_offset(&self, index: u32, need: u64) -> usize {
        let off = u64::from(index) * u64::from(self.stride);
        let len = self.entries.len() as u64;
        if off + need > len {
            panic!(
                "slot {index} (offset {off}, {need} bytes) past the {len}-byte entry store"
            );
        }
        off as usize
    }

    /// Allocates and installs a pool context: the creation routine.
    ///
    /// Allocates the 28-byte context; when that fails returns `None` (the
    /// original clears its context global and returns 0). Otherwise the
    /// block is initialised with the two arguments plus the tag 16, and the
    /// initialiser's answer is returned (the original publishes it into its
    /// context global and returns it too).
    pub fn create_ctx(
        a0: u32,
        a1: u32,
        alloc: &mut impl CtxAlloc,
        init: &mut impl CtxInit,
    ) -> Option<CtxHandle> {
        let block = alloc.alloc(CTX_SIZE)?;
        Some(init.init(block, a0, a1, INIT_TAG))
    }

    /// Whether slot `index` currently holds a live entry.
    ///
    /// Dead is sticky off the flag byte only: a set [`DEAD_BIT`] reads 0,
    /// anything else reads 1. The 32-bit form also requires the computed
    /// slot address to be non-zero; every pool this lift can build holds
    /// real entries, so that guard is always true here (see the registry).
    ///
    /// # Panics
    ///
    /// When `index` is past the flag store.
    #[must_use]
    pub fn is_occupied(&self, index: u32) -> bool {
        self.flag(index) & DEAD_BIT == 0
    }

    /// Data word of slot `index`: the word at entry + 4.
    ///
    /// # Panics
    ///
    /// When the slot is dead (the original reads through a null pointer
    /// and faults), when `index` is past the flag store, or when the word
    /// would leave the entry store.
    #[must_use]
    pub fn data_word(&self, index: u32) -> u32 {
        if self.flag(index) & DEAD_BIT != 0 {
            panic!("data word of dead slot {index}: the original faults through null");
        }
        let off = self.slot_offset(index, DATA_OFF + 4);
        u32::from_le_bytes(self.entries[off + 4..off + 8].try_into().unwrap())
    }

    /// Stores `value` into slot `index`'s head word, notifying on non-zero.
    ///
    /// Returns whether the head word is non-zero afterwards. The notifier
    /// runs on the index exactly when `value` is non-zero.
    ///
    /// # Panics
    ///
    /// When the slot is dead (the original stores through null and faults),
    /// when `index` is past the flag store, or when the head word would
    /// leave the entry store.
    pub fn assign(&mut self, index: u32, value: u32, notify: &mut impl Notify) -> bool {
        if self.flag(index) & DEAD_BIT != 0 {
            panic!("assign to dead slot {index}: the original faults through null");
        }
        let off = self.slot_offset(index, 4);
        self.entries[off..off + 4].copy_from_slice(&value.to_le_bytes());
        if value != 0 {
            notify.notify(index);
        }
        u32::from_le_bytes(self.entries[off..off + 4].try_into().unwrap()) != 0
    }

    /// Slot at byte `offset` from the entry base, or `None`.
    ///
    /// The 32-bit form takes an absolute slot *address* and answers 1 or 0
    /// in the low byte with arithmetic residue above it; the lift takes the
    /// address minus the base and answers the index. `offset` must lie in
    /// `[0, (count - 1) * stride]` (wrapping), must be a multiple of the
    /// stride under the original's *signed* division, and its flag byte
    /// must not have [`DEAD_BIT`] set.
    ///
    /// # Panics
    ///
    /// When the stride is 0 or the division overflows (the original faults
    /// with a divide error either way), when the quotient resolves below
    /// the flag store, or on an empty pool (the original reads through the
    /// pool's table pointer).
    #[must_use]
    pub fn slot_at_offset(&self, offset: u32) -> Option<usize> {
        let count = u32::try_from(self.flags.len())
            .expect("pools larger than u32::MAX slots are out of domain");
        let end = count.wrapping_sub(1).wrapping_mul(self.stride);
        if offset > end {
            return None;
        }
        let stride_i = self.stride as i32;
        let diff = offset as i32;
        if stride_i == 0 || (diff == i32::MIN && stride_i == -1) {
            panic!("slot offset {offset:#x} with stride {}: the original faults on the division", self.stride);
        }
        let index = diff / stride_i;
        let rem = diff % stride_i;
        if rem != 0 {
            return None;
        }
        if index < 0 {
            panic!("slot offset {offset:#x} resolves below the flag store");
        }
        let flag = *self.flags.get(index as usize).unwrap_or_else(|| {
            panic!("slot offset {offset:#x} resolves past the flag store")
        });
        if flag & DEAD_BIT != 0 {
            None
        } else {
            Some(index as usize)
        }
    }

    /// Steps `cursor` down to the next live slot, answering its index.
    ///
    /// A negative cursor restarts the walk from the slot count. Slots at
    /// and below the cursor are scanned downward, skipping dead flag bytes;
    /// the first live one is written back to the cursor and answered. When
    /// no live slot remains the cursor is set to -1 and `None` is answered.
    /// The 32-bit form answers the slot *address* or null; the address is
    /// `base + stride * index` with the original's wrapping, which the
    /// proof reconstructs per case.
    ///
    /// # Panics
    ///
    /// When the cursor starts above the slot count (the original reads past
    /// the flag store).
    pub fn cursor_step(&self, cursor: &mut i32) -> Option<usize> {
        if *cursor <= -1 {
            *cursor = i32::try_from(self.flags.len())
                .expect("pools larger than i32::MAX slots are out of domain");
        }
        let mut idx = cursor.wrapping_sub(1);
        if idx < 0 {
            *cursor = -1;
            return None;
        }
        loop {
            let flag = *self.flags.get(idx as usize).unwrap_or_else(|| {
                panic!(
                    "cursor {idx} past {} slots: the original reads past the flag store",
                    self.flags.len()
                )
            });
            if flag & DEAD_BIT == 0 {
                *cursor = idx;
                return Some(idx as usize);
            }
            idx = idx.wrapping_sub(1);
            if idx < 0 {
                *cursor = -1;
                return None;
            }
        }
    }

    /// Stores a table cell plus the successor index through `out`.
    ///
    /// Reads the successor index at slot `index`'s entry + `0x0c`, answering
    /// false (no call, no write) when it is -1. Otherwise runs the refresh
    /// for the table row and stores `table[scale * 25 + row + 0x16] +
    /// successor` (wrapping), answering true. The 32-bit form works the
    /// same sum in bytes against the refresh answer as an address
    /// (`scale * 100 + answer + 0x58`); every term is a whole number of
    /// words, so the word indexing matches exactly when no address wraps.
    ///
    /// # Panics
    ///
    /// When the slot is dead (the original faults through null), when
    /// `index` is past the flag store, when the successor word would leave
    /// the entry store, or when the cell index is past `table` (the
    /// original reads past it).
    pub fn indexed_store(
        &self,
        index: u32,
        scale: u32,
        table: &[u32],
        refresh: &mut impl Refresh,
        out: &mut u32,
    ) -> bool {
        if self.flag(index) & DEAD_BIT != 0 {
            panic!("indexed store of dead slot {index}: the original faults through null");
        }
        let off = self.slot_offset(index, NEXT_OFF + 4);
        let next = u32::from_le_bytes(self.entries[off + 12..off + 16].try_into().unwrap());
        if next == NO_NEXT {
            return false;
        }
        let row = refresh.refresh();
        let cell_index = scale
            .wrapping_mul(TABLE_ROW_WORDS)
            .wrapping_add(row)
            .wrapping_add(TABLE_BASE_WORDS);
        let cell = *table.get(cell_index as usize).unwrap_or_else(|| {
            panic!("table cell {cell_index} past {} words: the original reads past the table", table.len())
        });
        *out = cell.wrapping_add(next);
        true
    }
}
