//! Small structures: the allocation registry and the key table.
//!
//! Structures with one or two routines share this module: the registry of
//! allocated slots behind its counter ([`SlotRegistry`]) and the eight-id
//! key table ([`KeyTable`]).

use core::fmt::Debug;

/// Size of one registry entry, as the allocator is asked for it.
pub const REG_ENTRY_SIZE: usize = 8;

/// One initialised registry entry: eight opaque bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegEntry(pub [u8; REG_ENTRY_SIZE]);

/// The collaborator allocating and initialising registry entries.
pub trait RegBuild: Debug {
    /// An allocated-but-uninitialised block, as the allocator hands it back.
    type Pending;
    /// Allocates one entry block, or answers null.
    fn alloc(&mut self) -> Option<Self::Pending>;
    /// Initialises the block with `arg`, answering the entry's bytes.
    fn init(&mut self, block: Self::Pending, arg: u32) -> RegEntry;
}

/// The allocation registry: a counter plus the live entries and the
/// counter values whose allocation failed.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SlotRegistry {
    count: u32,
    live: Vec<RegEntry>,
    failed: Vec<u32>,
}

impl SlotRegistry {
    /// An empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A registry over caller-built state (the counter, the live entries
    /// in allocation order, the failed counter values in order).
    #[must_use]
    pub fn with_state(count: u32, live: Vec<RegEntry>, failed: Vec<u32>) -> Self {
        Self {
            count,
            live,
            failed,
        }
    }

    /// The counter.
    #[must_use]
    pub fn count(&self) -> u32 {
        self.count
    }

    /// The live entries, in allocation order.
    #[must_use]
    pub fn live(&self) -> &[RegEntry] {
        &self.live
    }

    /// The counter values whose allocation failed, in order.
    #[must_use]
    pub fn failed(&self) -> &[u32] {
        &self.failed
    }

    /// Allocates one entry and records it, answering the counter value
    /// before incrementing.
    ///
    /// On success the new entry is initialised through the collaborator
    /// and recorded; on failure a null marker is recorded. The counter
    /// increments (wrapping) either way.
    pub fn alloc_slot(&mut self, arg: u32, build: &mut impl RegBuild) -> u32 {
        let old = self.count;
        match build.alloc() {
            Some(block) => self.live.push(build.init(block, arg)),
            None => self.failed.push(old),
        }
        self.count = old.wrapping_add(1);
        old
    }
}

/// The eight-id key table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyTable {
    /// The slot ids, in scan order.
    pub ids: [u32; 8],
}

impl KeyTable {
    /// Finds the first slot whose id matches `key` (the original reads
    /// the key from its object's key word), or all-ones when none matches.
    #[must_use]
    pub fn find(&self, key: u32) -> u32 {
        for (i, id) in self.ids.iter().enumerate() {
            if *id == key {
                // Eight ids: the index fits in a word.
                #[allow(clippy::cast_possible_truncation)]
                let i = i as u32;
                return i;
            }
        }
        u32::MAX
    }
}
