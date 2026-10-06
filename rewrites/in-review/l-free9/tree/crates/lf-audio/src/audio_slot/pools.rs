//! Small audio-slot pools: one routine each, sharing this module.
//!
//! The strided pool ([`StridedPool`]) allocates fixed-stride elements and
//! links them into an index table; the triplet table ([`TripletTable`])
//! allocates slots in a fixed global table; the pointer array
//! ([`PtrArray`]) releases every live entry.

/// Stride of one pool element.
pub const POOL_STRIDE: u32 = 0x70;
/// Bytes per index-table cell (element word plus tag word).
pub const POOL_CELL: u32 = 8;
/// Entries in the global triplet table.
pub const TRIPLET_ENTRIES: usize = 32;
/// Words per triplet-table entry.
pub const TRIPLET_STRIDE: u32 = 20;
/// Triplet-table marker meaning "free".
pub const TRIPLET_FREE: u32 = 0xFFFF_FFFF;
/// Slots in the pointer array.
pub const PTR_COUNT: u32 = 0x40;

/// Sets up a fresh pool element: the strided pool's setup call.
pub trait PoolSetup {
    /// Initialises element `slot` (its index) with `tag` and `flags`.
    fn setup(&mut self, slot: u32, tag: u32, flags: u32);
}

impl<F: FnMut(u32, u32, u32)> PoolSetup for F {
    fn setup(&mut self, slot: u32, tag: u32, flags: u32) {
        self(slot, tag, flags);
    }
}

/// A strided audio pool: capacity, used count, element table.
///
/// The element store lives behind the pool-base global; its base travels
/// here as an opaque `u32` (arithmetic only, never dereferenced) so the
/// element words and the answer match the 32-bit bytes exactly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StridedPool {
    /// Capacity (words at `this+0x48`).
    cap: u32,
    /// Used count (`this+0x50`).
    count: u32,
    /// Element-store base (pool global), opaque.
    pool_base: u32,
    /// Index-table cells: element word plus tag word.
    table: Vec<(u32, u32)>,
}

impl StridedPool {
    /// A pool with `cap`, `count`, opaque `pool_base` and `table` cells.
    #[must_use]
    pub const fn from_parts(cap: u32, count: u32, pool_base: u32, table: Vec<(u32, u32)>) -> Self {
        Self {
            cap,
            count,
            pool_base,
            table,
        }
    }

    /// Allocates one slot and links it into the table.
    ///
    /// Fails with 0 when the used count has reached capacity. Otherwise
    /// bumps the count, sets up the new element at
    /// `pool_base + count * POOL_STRIDE` (wrapping), records the element
    /// word and tag in the next index-table cell, and answers the
    /// element word.
    ///
    /// # Panics
    ///
    /// When the next cell is past the modelled table (the original
    /// writes through the table pointer regardless).
    pub fn alloc(&mut self, tag: u32, flags: u32, setup: &mut impl PoolSetup) -> u32 {
        if self.count >= self.cap {
            return 0;
        }
        // The original bumps the count first, then sets up the element.
        let at = self.count;
        self.count = self.count.wrapping_add(1);
        let slot = self.pool_base.wrapping_add(at.wrapping_mul(POOL_STRIDE));
        setup.setup(at, tag, flags);
        let cell = self.table.get_mut(at as usize).unwrap_or_else(|| {
            panic!(
                "cell {at} past the {} modelled cells: the original writes on regardless",
                self.table.len()
            )
        });
        // The original stores the tag word first, then the element word.
        *cell = (slot, tag);
        slot
    }
}

/// One triplet-table entry: marker plus the stored pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triplet {
    /// Marker word (entry `+0`).
    pub marker: u32,
    /// First stored word (entry `+4`).
    pub first: u32,
    /// Second stored word (entry `+8`).
    pub second: u32,
}

/// The fixed global triplet table: 32 entries of five words.
///
/// Only the first three words of each entry are modelled; the last two
/// are never touched by the allocator and survive byte for byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TripletTable {
    /// Entries: a marker of [`TRIPLET_FREE`] means free, exactly like
    /// the 32-bit scan (an entry whose stored marker is free reads as
    /// free again).
    entries: [Triplet; TRIPLET_ENTRIES],
}

impl TripletTable {
    /// A table over `entries`.
    #[must_use]
    pub const fn from_entries(entries: [Triplet; TRIPLET_ENTRIES]) -> Self {
        Self { entries }
    }

    /// Allocates the first free entry and stores a triplet.
    ///
    /// Scans for the first entry whose marker is [`TRIPLET_FREE`]. On a
    /// hit stores marker `arg2`, `arg0`, `arg1` and answers the entry
    /// index; when every marker differs answers `None` (the 32-bit code
    /// answers `arg0` on a hit and the one-past-the-end address on a
    /// miss; the proof pins both shapes).
    pub fn alloc(&mut self, arg0: u32, arg1: u32, arg2: u32) -> Option<usize> {
        for (i, entry) in self.entries.iter_mut().enumerate() {
            if entry.marker == TRIPLET_FREE {
                *entry = Triplet {
                    marker: arg2,
                    first: arg0,
                    second: arg1,
                };
                return Some(i);
            }
        }
        None
    }
}

/// Releases one pointer-array slot: the array release's callee.
pub trait ReleaseSlot {
    /// Releases slot `index`.
    fn release(&mut self, index: u32);
}

impl<F: FnMut(u32)> ReleaseSlot for F {
    fn release(&mut self, index: u32) {
        self(index);
    }
}

/// A pointer array: 64 slots holding opaque pointer words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtrArray {
    /// Slot words: 0 means empty.
    slots: Vec<u32>,
}

impl PtrArray {
    /// An array over `slots`.
    #[must_use]
    pub const fn from_slots(slots: Vec<u32>) -> Self {
        Self { slots }
    }

    /// Releases every live entry: each nonzero slot runs the release
    /// call on its index, then is cleared. Answers 0.
    pub fn release_all(&mut self, release: &mut impl ReleaseSlot) -> u32 {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if *slot != 0 {
                release.release(i as u32);
                *slot = 0;
            }
        }
        0
    }
}
