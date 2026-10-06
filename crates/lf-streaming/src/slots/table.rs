//! The streaming slot table: an entry array with a capacity.
//!
//! Lifted from the verified rewrites. The 32-bit table object holds the
//! entry-array base at `+0x00` and a capacity at `+0x04`; entries are 24
//! bytes. The lift owns the entries as a vector; an empty vector is the
//! null base, and the capacity stays a signed word because the range
//! check reads it signed.

use crate::slots::entry::ENTRY_LEN;
use crate::slots::entry::KindBytes;
use crate::slots::entry::KindSlots;
use crate::slots::entry::StreamEntry;

/// Flag bits the mask test always checks, whatever the caller mask is.
const ALWAYS_TEST: u32 = 0xc6;
/// Index value that never names a usable slot.
const NO_SLOT: u32 = 0xffff;
/// Slack the range check allows past the capacity.
const RANGE_SLACK: i32 = 6;
/// Kind-table stride of the kind flag: bytes per value-kind.
const KIND_FLAG_STRIDE: u32 = 160;
/// Magic multiplier of the signed divide by 24.
const DIV24_MAGIC: i64 = 0x2aaa_aaab;

/// The streaming slot table: owned entries plus a signed capacity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotTable {
    /// Entries in index order; empty is the 32-bit null base.
    entries: Vec<StreamEntry>,
    /// Capacity word, read signed by the range check.
    cap: i32,
}

impl SlotTable {
    /// Builds a table from its entries in index order and its capacity.
    #[must_use]
    pub const fn from_parts(entries: Vec<StreamEntry>, cap: i32) -> Self {
        Self { entries, cap }
    }

    /// Entries in index order.
    #[must_use]
    pub fn entries(&self) -> &[StreamEntry] {
        &self.entries
    }

    /// The capacity word.
    #[must_use]
    pub fn cap(&self) -> i32 {
        self.cap
    }

    /// Whether slot `idx` is a usable stream slot.
    ///
    /// Restates `stream_slot_in_range`: usable when the capacity is
    /// non-negative, the base is non-null (the lift: the table holds
    /// entries), the index is non-negative and not the `0xffff`
    /// sentinel, and the index is below `capacity + 6` (signed,
    /// wrapping add). The 1/0 answer narrows to bool.
    #[must_use]
    // The wrap is the behaviour: the original compares the index signed.
    #[allow(clippy::cast_possible_wrap)]
    pub fn is_slot_usable(&self, idx: u32) -> bool {
        if self.cap < 0 {
            return false;
        }
        if self.entries.is_empty() {
            return false;
        }
        let signed = idx as i32;
        if idx == NO_SLOT || signed < 0 {
            return false;
        }
        if signed >= self.cap.wrapping_add(RANGE_SLACK) {
            return false;
        }
        true
    }

    /// Whether entry `idx` has none of the flag bits in `mask | 0xc6` set.
    ///
    /// Restates `stream_entry_test_mask`, which reads the flag word at
    /// `+0x0e` of the 24-byte entry.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned entry; the original reads past its array.
    #[must_use]
    pub fn test_mask(&self, idx: u32, mask: u32) -> bool {
        let entry = self.entry_at(idx);
        u32::from(entry.flags()) & (mask | ALWAYS_TEST) == 0
    }

    /// Whether entry `idx` of the table is active.
    ///
    /// Restates `stream_table_entry_is_active`: inactive when the table
    /// base is null (the lift: no entries) or the entry fails the
    /// active test. The 1/0 answer narrows to bool.
    ///
    /// # Panics
    ///
    /// When the table holds entries but `idx` names none; the original
    /// reads past its array.
    #[must_use]
    pub fn entry_is_active(&self, idx: u32) -> bool {
        if self.entries.is_empty() {
            return false;
        }
        self.entry_at(idx).is_active()
    }

    /// Kind flag byte of entry `idx`, with its scaled-index residue.
    ///
    /// Restates `stream_entry_kind_flag`: the value word's low byte of
    /// the 24-byte entry is scaled by 160, selects a flag byte from the
    /// kind table, and the answer keeps the scaled index in its upper
    /// 24 bits with the flag byte in the low byte. The composition is
    /// exact portable arithmetic, so the answer is not narrowed.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned entry; the original reads past its array.
    #[must_use]
    pub fn kind_flag(&self, idx: u32, kinds: &KindBytes) -> u32 {
        let scaled = u32::from(self.entry_at(idx).value_kind()).wrapping_mul(KIND_FLAG_STRIDE);
        (scaled & 0xffff_ff00) | u32::from(kinds.bytes[scaled as usize])
    }

    /// Sets bit 15 of entry `idx`'s flag word.
    ///
    /// Restates `stream_entry_set_flag_8000`, which ORs `0x8000` into
    /// the flag word at `+0x0e`. The returned table base narrows away:
    /// callers own the table.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned entry; the original writes past its array.
    pub fn set_flag_bit15(&mut self, idx: u32) {
        let at = self.checked_index(idx);
        self.entries[at].set_flag_bit15();
    }

    /// Slot number of entry `idx` relative to its kind's base slot.
    ///
    /// Restates `stream_entry_slot_index`: the entry's index is
    /// `(entry - table_base) / 24` (signed, through the original's
    /// magic multiply), truncated to 16 bits, minus the kind's base
    /// slot from the kind table indexed by the kind byte times 100.
    /// The entry address narrows to its index: the byte offset is
    /// `idx * 24`, and the magic sequence runs on it exactly.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned entry; the original reads past its array.
    #[must_use]
    // The casts are the behaviour: the original reinterprets the offset
    // signed and truncates the quotient to 16 bits.
    #[allow(
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn relative_slot(&self, idx: u32, kinds: &KindSlots) -> u32 {
        let offset = idx.wrapping_mul(ENTRY_LEN as u32);
        let index = u32::from(div24(offset as i32) as u16);
        let kind = self.entry_at(idx).kind();
        let base = kinds.slots[usize::from(kind)];
        index.wrapping_sub(base)
    }

    /// Entry `idx`, or a panic naming the miss.
    fn entry_at(&self, idx: u32) -> StreamEntry {
        self.entries[self.checked_index(idx)]
    }

    /// Vec position of entry `idx`, panicking past the owned entries.
    fn checked_index(&self, idx: u32) -> usize {
        usize::try_from(idx)
            .ok()
            .filter(|at| *at < self.entries.len())
            .unwrap_or_else(|| panic!("slot index {idx} past {} owned entries", self.entries.len()))
    }
}

/// Signed divide by 24 through the original's magic sequence.
///
/// The multiplier, shift and sign fix-up are the original's own steps,
/// restated as portable arithmetic.
// The casts are the behaviour: the sequence reinterprets high halves.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
const fn div24(dividend: i32) -> i32 {
    let high = ((dividend as i64 * DIV24_MAGIC) >> 32) as i32;
    let shifted = high >> 2;
    shifted.wrapping_add(((shifted as u32) >> 31) as i32)
}
