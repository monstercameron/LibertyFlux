//! The composer's optimised data block: the bulk tables of a composer,
//! cleared and rebuilt as a unit, and the destructor that tears it down.
//!
//! The block holds a value table and a pointer table. The value table has
//! [`VALUE_COUNT`] entries. Each entry's two words are cleared by a reset;
//! in the original the entries are [`VALUE_STRIDE`] bytes apart, and the
//! other bytes of each stride belong to fields this module does not hold.
//! The pointer table has [`TABLE_COUNT`] entries, [`TABLE_STRIDE`] bytes
//! apart. A reset makes each entry hold the block-relative offset of a
//! point [`TABLE_TARGET_SKEW`] bytes past the entry's own start. The
//! block's count word is cleared last.
//!
//! Block-relative offsets are the only addresses here: an offset is what
//! the original's word means once the block's own address is subtracted
//! away. Callees and the thread's manager are traits, so the lifted code
//! names no address and no slot. The verified rewrites are proven against
//! this module by the `lf-composerdiff` test crate (three levels up to the
//! repository root).

use lf_core::boundary::Handle32;

/// Entries in the value table.
pub const VALUE_COUNT: usize = 64;
/// Byte distance between value entries in the original.
pub const VALUE_STRIDE: u32 = 0x34;
/// Block-relative offset of the first word of the first value entry.
pub const VALUES_OFFSET: u32 = 0xE48;
/// Entries in the pointer table.
pub const TABLE_COUNT: usize = 63;
/// Block-relative offset of the first pointer-table entry.
pub const TABLE_OFFSET: u32 = 0x20;
/// Byte distance between pointer-table entries in the original.
pub const TABLE_STRIDE: u32 = 0x30;
/// Distance past its own start that each pointer-table entry points.
pub const TABLE_TARGET_SKEW: u32 = 0x2C;
/// Block-relative offset of the header that the reset releases.
pub const HEADER_OFFSET: u32 = 4;
/// Block-relative offset of the count word that the reset clears.
pub const COUNT_OFFSET: u32 = 0xBF0;
/// Flag bit that asks the destructor to free the block through the manager.
pub const FREE_FLAG: u32 = 1;

/// Identity of one optimised data block.
pub type BlockId = Handle32<ComposerOptimizedData>;

/// One value-table entry. Its two words are cleared by a reset; their
/// meaning is not established by the verified rewrites.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ValueEntry {
    /// The entry's first word.
    pub first: u32,
    /// The entry's second word.
    pub second: u32,
}

/// One pointer-table entry: a block-relative offset.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TableEntry {
    /// Block-relative offset the entry points at.
    pub target: u32,
}

/// Callees of the composer data block. Each is a method of the original
/// that this lane does not lift.
pub trait ComposerDataCallees {
    /// Callee of the destructor: called on the block itself. The verified
    /// rewrite does not establish its meaning; the evidence proposes a
    /// composer reset, and that name is not accepted.
    fn reset_optimized(&mut self, block: Option<BlockId>);

    /// Callee of the table reset: called on the block's header, at
    /// `offset` bytes past the block. The evidence proposes a frame-release
    /// name, which is not accepted.
    fn release_child_frames(&mut self, block: Option<BlockId>, offset: u32);
}

/// The thread's block manager, which frees a block.
pub trait ObjectManager {
    /// Frees `block` through the manager.
    fn free_object(&mut self, block: BlockId);
}

/// The optimised data block: its value table, pointer table and count.
pub struct ComposerOptimizedData {
    values: [ValueEntry; VALUE_COUNT],
    table: [TableEntry; TABLE_COUNT],
    count: u32,
}

impl Default for ComposerOptimizedData {
    fn default() -> Self {
        Self {
            values: [ValueEntry::default(); VALUE_COUNT],
            table: [TableEntry::default(); TABLE_COUNT],
            count: 0,
        }
    }
}

impl ComposerOptimizedData {
    /// A block with the given tables and count.
    #[must_use]
    pub fn new(values: [ValueEntry; VALUE_COUNT], table: [TableEntry; TABLE_COUNT], count: u32) -> Self {
        Self {
            values,
            table,
            count,
        }
    }

    /// The value table.
    #[must_use]
    pub fn values(&self) -> &[ValueEntry; VALUE_COUNT] {
        &self.values
    }

    /// The pointer table.
    #[must_use]
    pub fn table(&self) -> &[TableEntry; TABLE_COUNT] {
        &self.table
    }

    /// The count word.
    #[must_use]
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Resets the tables: the header is released through its callee, every
    /// value entry is cleared, every pointer-table entry is rebuilt to point
    /// [`TABLE_TARGET_SKEW`] bytes past itself, and the count is cleared.
    ///
    /// Returns the offset that the last pointer-table entry now holds.
    pub fn reset_tables<C: ComposerDataCallees + ?Sized>(
        &mut self,
        block: Option<BlockId>,
        callees: &mut C,
    ) -> u32 {
        callees.release_child_frames(block, HEADER_OFFSET);
        for entry in &mut self.values {
            *entry = ValueEntry::default();
        }
        for (index, entry) in self.table.iter_mut().enumerate() {
            entry.target = TABLE_OFFSET + (index as u32) * TABLE_STRIDE + TABLE_TARGET_SKEW;
        }
        self.count = 0;
        self.table[TABLE_COUNT - 1].target
    }

    /// The scalar deleting destructor: the block is torn down through its
    /// callee, and when the free flag is set and the block has an identity,
    /// it is freed through the manager. Returns the identity.
    ///
    /// The lift holds no memory of its own to free: the owner that holds the
    /// block drops it after this call when the manager has freed it.
    ///
    /// # Panics
    ///
    /// When a block is to be freed and no manager is given. The original
    /// would call through a missing manager, so this case is outside the
    /// domain.
    pub fn scalar_deleting_destructor<C, M>(
        block: Option<BlockId>,
        flags: u32,
        callees: &mut C,
        manager: Option<&mut M>,
    ) -> Option<BlockId>
    where
        C: ComposerDataCallees + ?Sized,
        M: ObjectManager + ?Sized,
    {
        callees.reset_optimized(block);
        if flags & FREE_FLAG != 0 {
            if let Some(block) = block {
                let manager = manager.expect("free asked for without a manager");
                manager.free_object(block);
            }
        }
        block
    }
}
