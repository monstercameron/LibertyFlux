//! The six table slots: one generic function per slot.
//!
//! Every slot starts by fetching its board's tables through
//! [`BoardTables::fetch`] with the board id from its [`LeaderboardDesc`][crate::LeaderboardDesc].
//! The fetch block layout of the 32-bit form (which word holds the count,
//! which the table pointers) stays at the boundary: the differential test's
//! fetch stub fills it, and the lifted side sees only [`BoardView`].

use crate::desc::LeaderboardDesc;

/// "Not found" answer shared by the lookup slots (`-1` as `u32`).
pub const NOT_FOUND: u32 = 0xFFFF_FFFF;

/// One board's fetched tables: what the registry returns for a board id.
///
/// `count` is the registry's entry count word (read signed by some slots,
/// unsigned by others, exactly as the original reads it). `primary` is the
/// id table for the single-table slots and the key-source table for the
/// two-table slots; `secondary` is the second table where the slot uses one.
#[derive(Clone, Copy, Debug)]
pub struct BoardView<'a> {
    /// The registry's count word.
    pub count: u32,
    /// The primary table.
    pub primary: &'a [u32],
    /// The second table, for the two-table slots.
    pub secondary: Option<&'a [u32]>,
}

/// Collaborators behind the table slots: the board registry and the entry
/// classifier (the 32-bit form's fetch and classify callees).
pub trait BoardTables {
    /// Fetches one board's tables; `None` when the registry reports failure
    /// (the 32-bit form's zero low byte).
    fn fetch(&mut self, board: u32) -> Option<BoardView<'_>>;

    /// Classifies one entry id (the 32-bit form's classify callee).
    fn classify(&mut self, value: u32) -> u32;
}

/// Reads one table word. The original reads wherever its index says,
/// validated or not; the lift's stated domain is an index inside the table.
///
/// # Panics
///
/// When `index` is outside `table`, naming the caller. Every differential
/// case stays inside the domain; out-of-range reads are not modelled.
fn word(table: &[u32], index: u32, caller: &str) -> u32 {
    *table.get(index as usize).unwrap_or_else(|| {
        panic!(
            "{caller}: table index {index} outside {} words",
            table.len()
        )
    })
}

/// The second table of a two-table slot.
///
/// # Panics
///
/// When the view carries none (a registry/boundary bug: the fetch stub
/// always provides both tables for these slots).
fn second<'a>(view: &BoardView<'a>, caller: &str) -> &'a [u32] {
    view.secondary
        .unwrap_or_else(|| panic!("{caller}: two-table slot fetched a view without a second table"))
}

/// Slot vf6: index of `key` in the board's id table, or [`NOT_FOUND`].
///
/// Fetch failure, or a count that is not positive read as signed, finds
/// nothing. Otherwise the table scans from index 0 while the signed index
/// stays below the signed count; the first match wins.
pub fn find_index(store: &mut impl BoardTables, desc: &LeaderboardDesc, key: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return NOT_FOUND;
    };
    let count = view.count.cast_signed();
    if count <= 0 {
        return NOT_FOUND;
    }
    let mut i = 0u32;
    loop {
        if word(view.primary, i, "find_index") == key {
            return i;
        }
        i = i.wrapping_add(1);
        if i.cast_signed() >= count {
            return NOT_FOUND;
        }
    }
}

/// Slot vf7: the id stored at `index`, or [`NOT_FOUND`] when the fetch fails.
///
/// The original checks no bounds; the lift's domain is `index` inside the
/// table (see [`word`]).
pub fn fetch_id(store: &mut impl BoardTables, desc: &LeaderboardDesc, index: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return NOT_FOUND;
    };
    word(view.primary, index, "fetch_id")
}

/// Slot vf8: class tag of the entry at `index`.
///
/// The table word at `index` is classified and the answer maps
/// 1 to 4, 2 and 3 to 8, 5 to 4, and anything else to 0. Fetch failure
/// maps to 0, the same default as an unlisted answer.
pub fn class_tag(store: &mut impl BoardTables, desc: &LeaderboardDesc, index: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return 0;
    };
    let entry = word(view.primary, index, "class_tag");
    match store.classify(entry) {
        1 => 4,
        2 | 3 => 8,
        5 => 4,
        _ => 0,
    }
}

/// Slot vf9: class rank of the entry at `index`.
///
/// Like [`class_tag`] with a different map: 1 to 0, 2 to 1, 3 to 3, 4 to
/// [`NOT_FOUND`], 5 to 2, and anything else (fetch failure included) to
/// [`NOT_FOUND`].
pub fn class_rank(store: &mut impl BoardTables, desc: &LeaderboardDesc, index: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return NOT_FOUND;
    };
    let entry = word(view.primary, index, "class_rank");
    match store.classify(entry) {
        1 => 0,
        2 => 1,
        3 => 3,
        4 => NOT_FOUND,
        5 => 2,
        _ => NOT_FOUND,
    }
}

/// Slot vf12: find `primary[index]` inside the second table.
///
/// The key is the primary table at `index`. A key of [`NOT_FOUND`], a zero
/// count (read unsigned), fetch failure, or a failed linear scan of the
/// second table (unsigned bound) all answer [`NOT_FOUND`]; otherwise the
/// first matching index in the second table.
pub fn reverse_lookup(store: &mut impl BoardTables, desc: &LeaderboardDesc, index: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return NOT_FOUND;
    };
    let key = word(view.primary, index, "reverse_lookup");
    if key == NOT_FOUND {
        return NOT_FOUND;
    }
    let count = view.count;
    if count == 0 {
        return NOT_FOUND;
    }
    let table_b = second(&view, "reverse_lookup");
    let mut i = 0u32;
    loop {
        if word(table_b, i, "reverse_lookup") == key {
            return i;
        }
        i = i.wrapping_add(1);
        if i >= count {
            return NOT_FOUND;
        }
    }
}

/// Slot vf13: find `key` in the primary table, return the second table there.
///
/// Fetch failure, or a count that is not positive read as signed, answers
/// [`NOT_FOUND`]. Otherwise the primary table scans for `key` (signed
/// bound, first match wins) and the answer is the second table at the
/// found index, or [`NOT_FOUND`] when the key is absent.
pub fn joined_fetch(store: &mut impl BoardTables, desc: &LeaderboardDesc, key: u32) -> u32 {
    let Some(view) = store.fetch(desc.board_id) else {
        return NOT_FOUND;
    };
    let count = view.count.cast_signed();
    if count <= 0 {
        return NOT_FOUND;
    }
    let table_b = second(&view, "joined_fetch");
    let mut i = 0u32;
    let idx = loop {
        if word(view.primary, i, "joined_fetch") == key {
            break i;
        }
        i = i.wrapping_add(1);
        if i.cast_signed() >= count {
            return NOT_FOUND;
        }
    };
    word(table_b, idx, "joined_fetch")
}
