//! The streaming control block: flag banks and 8-byte entries.
//!
//! Lifted from the verified rewrites. One block serves three routines:
//! two match instances that compare a caller key against a slot's
//! entry when the slot's flag bank enables it (one generic method,
//! proved against both instances), and a reset that notifies, releases
//! the slot's records, and clears the slot.

/// Which flag bank enables a slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagBank {
    /// The first bank (base `+0xF0`).
    A,
    /// The second bank (base `+0xFE`).
    B,
}

/// The streaming control block: two flag banks over 8-byte entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlBlock {
    /// First flag bank in slot order.
    pub flags_a: Vec<u8>,
    /// Second flag bank in slot order.
    pub flags_b: Vec<u8>,
    /// Entries in slot order.
    pub entries: Vec<[u8; 8]>,
}

impl ControlBlock {
    /// Compares a caller key against slot `idx`'s entry, when enabled.
    ///
    /// Restates `stream_slot_match_f0` (bank A) and
    /// `stream_slot_match_fe` (bank B): when the flag byte of the
    /// bank is clear the answer is false without calling out;
    /// otherwise the comparison runs over the entry and the key, and
    /// the answer is true exactly when it reports equality (0). The
    /// 1/0 answer narrows to bool.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned slot; the original reads past its block.
    pub fn match_slot<C: SlotCompare>(
        &self,
        key: u32,
        idx: u32,
        bank: FlagBank,
        compare: &mut C,
    ) -> bool {
        let at = self.checked_index(idx);
        let flags = match bank {
            FlagBank::A => &self.flags_a,
            FlagBank::B => &self.flags_b,
        };
        if flags[at] == 0 {
            return false;
        }
        compare.compare(self.entries[at], key) == 0
    }

    /// Resets slot `idx`: notifies, releases its records, clears it.
    ///
    /// Restates `stream_slot_reset`: the watcher is notified of the
    /// slot, the two records owned by the slot's entry pair are
    /// released through the releaser, then the slot's flag byte is
    /// cleared and its 8-byte entry zeroed. The 0 answer narrows to
    /// unit. The two record addresses narrow to their block-relative
    /// offsets (`(idx + 2) * 8` and `(idx + 0x10) * 8`, wrapping); the
    /// proof rebuilds the addresses per case.
    ///
    /// # Panics
    ///
    /// When `idx` names no owned slot; the original reads and writes
    /// past its block.
    pub fn reset_slot<W: SlotWatcher, R: RecordRelease>(
        &mut self,
        idx: u32,
        watcher: &mut W,
        release: &mut R,
    ) {
        let at = self.checked_index(idx);
        watcher.notify(idx, 1);
        release.release(idx.wrapping_add(2).wrapping_mul(8));
        release.release(idx.wrapping_add(0x10).wrapping_mul(8));
        self.flags_a[at] = 0;
        self.entries[at] = [0; 8];
    }

    /// Slot position of `idx`, panicking past the owned slots.
    fn checked_index(&self, idx: u32) -> usize {
        usize::try_from(idx)
            .ok()
            .filter(|at| *at < self.entries.len())
            .unwrap_or_else(|| panic!("slot index {idx} past {} owned slots", self.entries.len()))
    }
}

/// Compares a slot entry against a caller key: the match callee.
///
/// The 32-bit callee takes the entry address and the key and answers
/// a difference word (0 is equal); the lift passes the entry itself
/// and keeps the difference word so the proof can script it.
pub trait SlotCompare {
    /// Answers the difference word of `entry` and `key`.
    fn compare(&mut self, entry: [u8; 8], key: u32) -> u32;
}

/// Watches slot resets: the reset's notify callee.
pub trait SlotWatcher {
    /// Records the reset of slot `idx` with `code`.
    fn notify(&mut self, idx: u32, code: u32);
}

/// Releases a slot's records: the reset's release callee.
///
/// The 32-bit callee takes the record address; the lift passes the
/// block-relative offset, and the proof rebuilds the address per case.
pub trait RecordRelease {
    /// Releases the record at `offset`.
    fn release(&mut self, offset: u32);
}

impl<F: FnMut([u8; 8], u32) -> u32> SlotCompare for F {
    fn compare(&mut self, entry: [u8; 8], key: u32) -> u32 {
        self(entry, key)
    }
}

impl<F: FnMut(u32, u32)> SlotWatcher for F {
    fn notify(&mut self, idx: u32, code: u32) {
        self(idx, code);
    }
}

impl<F: FnMut(u32)> RecordRelease for F {
    fn release(&mut self, offset: u32) {
        self(offset);
    }
}
