//! The blip marker table: handle lookup, flag-gated rows, kind gate.
//!
//! Lifted from the two verified `script_vm_entity_apply_offset` and
//! `script_vm_entity_read_offset` rewrites. Both resolve a marker handle
//! through a lookup callee to a row index into a table of row pointers.
//! The apply routine picks the row, or the fallback row when the row's
//! flag byte is clear, and forwards a packed position only for marker
//! kinds 4, 5 and 7 (Unknown: what the kinds mean). The read routine
//! copies one of two stored positions into a four-word output slot,
//! zeroing the slot on a lookup miss. The table's row pointers and the
//! fallback id become owned data; the lookup and the apply callee become
//! traits.

/// Byte offset of the flag byte inside a marker row.
pub const FLAG_OFF: usize = 8;
/// Byte offset of the kind word inside a marker row.
pub const KIND_OFF: usize = 0x48;
/// Byte offset of the flagged position inside a marker row.
pub const POS_A_OFF: usize = 0x30;
/// Byte offset of the unflagged position inside a marker row.
pub const POS_B_OFF: usize = 0x20;
/// Tag word the apply routine passes ahead of the handle.
pub const APPLY_TAG: u32 = 2;
/// Lookup answer meaning "no such marker" to the read routine.
pub const LOOKUP_MISS: u32 = 0xFFFF_FFFF;

/// Resolves a marker handle to a row index: the lookup callee.
pub trait BlipLookup {
    /// Answers the row index for `handle` (negative when signed means
    /// "no such marker" to the apply routine).
    fn lookup(&mut self, handle: u32) -> u32;
}

/// Applies a packed position to a marker: the apply callee.
pub trait BlipApply {
    /// Applies `pos` to `handle` under `tag`.
    fn apply(&mut self, tag: u32, handle: u32, pos: [u32; 3]);
}

impl<F: FnMut(u32) -> u32> BlipLookup for F {
    fn lookup(&mut self, handle: u32) -> u32 {
        self(handle)
    }
}

impl<F: FnMut(u32, u32, [u32; 3])> BlipApply for F {
    fn apply(&mut self, tag: u32, handle: u32, pos: [u32; 3]) {
        self(tag, handle, pos);
    }
}

/// One marker row: the flag byte, the kind word and the two positions.
///
/// The 32-bit row holds the flag at [`FLAG_OFF`], the kind at
/// [`KIND_OFF`], three words at [`POS_A_OFF`] and two at [`POS_B_OFF`];
/// the lift owns those fields and nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlipRow {
    /// Whether the flag byte is set: selects the row over the fallback
    /// (apply) and the flagged position over the unflagged one (read).
    pub flag: bool,
    /// The kind word: only 4, 5 and 7 apply.
    pub kind: u32,
    /// The position read when the flag byte is set.
    pub pos_a: [u32; 3],
    /// The first two words read when the flag byte is clear.
    pub pos_b: [u32; 2],
}

/// The marker table: row pointers resolved to owned rows.
///
/// The 32-bit routines read row pointers from a table and the fallback
/// index from a global word; the lift owns the rows as a vector and the
/// fallback as an index. Indexes past the store panic; the original
/// reads past it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlipTable {
    /// The rows, in table order.
    rows: Vec<BlipRow>,
    /// The fallback row index the apply routine uses when the resolved
    /// row's flag byte is clear.
    fallback: u32,
}

impl BlipTable {
    /// Builds the table over `rows` with the `fallback` index.
    #[must_use]
    pub fn new(rows: Vec<BlipRow>, fallback: u32) -> Self {
        Self { rows, fallback }
    }

    /// The rows, in table order.
    #[must_use]
    pub fn rows(&self) -> &[BlipRow] {
        &self.rows
    }

    /// The fallback row index.
    #[must_use]
    pub fn fallback(&self) -> u32 {
        self.fallback
    }

    /// Whether the kind word applies: 4, 5 or 7, nothing else.
    fn applies(kind: u32) -> bool {
        matches!(kind, 4 | 5 | 7)
    }

    /// Forwards a packed position for kinds 4, 5 and 7.
    ///
    /// A negative lookup answer (when signed) returns at once. Otherwise
    /// the row whose flag byte is set supplies the kind word: the
    /// resolved row when its flag is set, else the fallback row. Only
    /// kinds 4, 5 and 7 reach the apply callee, with
    /// ([`APPLY_TAG`], `handle`, the packed triple).
    pub fn apply_offset(
        &self,
        lookup: &mut impl BlipLookup,
        apply: &mut impl BlipApply,
        handle: u32,
        x: u32,
        y: u32,
        z: u32,
    ) {
        let id = lookup.lookup(handle);
        if (id as i32) < 0 {
            return;
        }
        let row = &self.rows[id as usize];
        let krow = if row.flag {
            row
        } else {
            &self.rows[self.fallback as usize]
        };
        if !Self::applies(krow.kind) {
            return;
        }
        apply.apply(APPLY_TAG, handle, [x, y, z]);
    }

    /// Reads a marker position into four words.
    ///
    /// A [`LOOKUP_MISS`] answer zeroes the first three words. Otherwise
    /// the resolved row supplies the flagged triple when its flag byte
    /// is set, else the unflagged pair with a zero third word. The
    /// fourth word is always zero: the original reads it from below its
    /// own frame (uninitialized scratch), and the verified contract pins
    /// that slot to zero.
    #[must_use]
    pub fn read_offset(&self, lookup: &mut impl BlipLookup, handle: u32) -> [u32; 4] {
        let id = lookup.lookup(handle);
        if id == LOOKUP_MISS {
            return [0, 0, 0, 0];
        }
        let row = &self.rows[id as usize];
        let (a, b, c) = if row.flag {
            (row.pos_a[0], row.pos_a[1], row.pos_a[2])
        } else {
            (row.pos_b[0], row.pos_b[1], 0)
        };
        [a, b, c, 0]
    }
}
