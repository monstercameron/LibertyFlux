//! The row-collector slot (vf14): gather one leaderboard's rows for a query.
//!
//! The collector scans `rows` rows (a per-board descriptor parameter).
//! Each row resolves to a key through the object's row slot, may be
//! skipped by the gate, then advances a cursor by [`ELEM`] or zero
//! depending on its cell's class. The distinguished row (the object's
//! picked slot) copies its item's bytes to `id_out` and sets the flag;
//! every other row writes through the store helper and records its bit in
//! the mask. Any failed row ends the scan with a false result.

use crate::desc::LeaderboardDesc;

/// Cursor advance for a wide row (classes 1, 2, 3 and 5).
pub const ELEM: u32 = 8;

/// Opaque manager cookie: the `board`/`store` word the helpers take,
/// owned by code not yet lifted (carried, never interpreted).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Manager(u32);

impl Manager {
    /// Wraps a raw cookie.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// The raw cookie, for the boundary only.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// Opaque reference to a fetched item, handed back to [`RowStore`].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ItemToken(u32);

impl ItemToken {
    /// Wraps a raw token.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// The raw token, for the boundary only.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// One board's key table: what the fetch yields for the collector.
#[derive(Clone, Copy, Debug)]
pub struct RowTable<'a> {
    /// The key table, indexed by row key.
    pub cells: &'a [u32],
}

/// The object's own two virtuals used by the collector: the distinguished
/// row and the row-to-key map. (In the full lift these become methods on
/// the leaderboard object; a trait keeps them fakeable here.)
pub trait RowPicker {
    /// The distinguished row index.
    fn picked(&self) -> u32;

    /// The key of one row.
    fn row_key(&self, row: u32) -> u32;
}

/// The six helpers behind the collector (the 32-bit form's callees).
///
/// All methods take `&self`: the collector holds the fetched table across
/// the loop while calling the other helpers, so exclusive borrows cannot
/// work. That fits the collaborators, which are read-model queries; fakes
/// log through the shared call recorder.
pub trait RowStore {
    /// Fetches one board's key table; `None` on registry failure.
    fn fetch(&self, board: u32) -> Option<RowTable<'_>>;

    /// The gate: true skips the row entirely.
    fn skip_row(&self, manager: Manager, key: u32) -> bool;

    /// Classifies one cell.
    fn classify(&self, cell: u32) -> u32;

    /// The item helper: `None` for a null item.
    fn item(&self, manager: Manager, key: u32) -> Option<ItemToken>;

    /// The length helper on an item token.
    fn item_len(&self, item: ItemToken) -> u32;

    /// The item's eight data bytes (the 32-bit form's direct read at
    /// item word +4; bundled here because native code has no image).
    fn item_bytes(&self, item: ItemToken) -> [u8; 8];

    /// The write helper: records the row at the pre-advance cursor.
    fn write(&self, manager: Manager, key: u32, at: u32, len: u32) -> bool;
}

/// Reads one table cell. The original reads wherever its key says; the
/// lift's stated domain is a key inside the table.
///
/// # Panics
///
/// When `key` is outside `cells`. Every differential case stays inside.
fn cell(cells: &[u32], key: u32) -> u32 {
    *cells
        .get(key as usize)
        .unwrap_or_else(|| panic!("collect: cell key {key} outside {} words", cells.len()))
}

/// The mask words for one written row: bit `row` in the low word below
/// row 32, in the high word below row 64, nowhere above.
fn completion_bit(row: u32) -> (u32, u32) {
    if row < 32 {
        (1u32 << row, 0)
    } else if row < 64 {
        (0, 1u32 << (row & 31))
    } else {
        (0, 0)
    }
}

/// Slot vf14: collect the board's rows for a stat query.
///
/// `cursor`/`size` bound a byte range (`limit = size + cursor`, wrapping,
/// compared unsigned). `id_out` takes the distinguished row's eight item
/// bytes (left untouched when no distinguished row is found),
/// `mask_out` the two-word one-hot mask of the last written row,
/// `flag_out` the found byte. Returns true only if every visited row
/// succeeded.
///
/// The length check is signed (`len.cast_signed() <= ELEM.cast_signed()`), as the
/// original's is, so a length above `i32::MAX` counts as negative and
/// passes. Writing this function is what exposed 144 verified rewrites of
/// the slot that compared unsigned: their contracts never scripted a
/// negative length, so the checker could not tell. They were withdrawn on
/// 5 October 2026 and re-proven with the signed comparison, with contracts
/// that script negative and edge lengths.
#[allow(clippy::too_many_arguments)]
pub fn collect(
    store: &impl RowStore,
    picker: &impl RowPicker,
    desc: &LeaderboardDesc,
    manager: Manager,
    cursor: u32,
    size: u32,
    id_out: &mut [u8; 8],
    mask_out: &mut [u8; 8],
    flag_out: &mut u8,
) -> bool {
    *mask_out = [0; 8];
    *flag_out = 0;
    let limit = size.wrapping_add(cursor);
    let mut pos = cursor;
    let picked = picker.picked();
    let Some(table) = store.fetch(desc.board_id) else {
        return false;
    };
    let mut live = true;
    let mut row = 0u32;
    while row < desc.rows {
        if !live {
            break;
        }
        let key = picker.row_key(row);
        if !store.skip_row(manager, key) {
            let class = store.classify(cell(table.cells, key));
            let adv: u32 = match class {
                1 | 2 | 3 | 5 => ELEM,
                _ => 0,
            };
            if picked == row {
                let mut found = false;
                if let Some(item) = store.item(manager, key) {
                    let len = store.item_len(item);
                    if len.cast_signed() <= ELEM.cast_signed() {
                        *id_out = store.item_bytes(item);
                        found = true;
                    }
                }
                *flag_out = u8::from(found);
                live = found;
            } else {
                let old = pos;
                pos = pos.wrapping_add(adv);
                if pos > limit {
                    live = false;
                } else if !store.write(manager, key, old, adv) {
                    live = false;
                } else {
                    let (lo, hi) = completion_bit(row);
                    mask_out[0..4].copy_from_slice(&lo.to_le_bytes());
                    mask_out[4..8].copy_from_slice(&hi.to_le_bytes());
                    live = true;
                }
            }
        }
        row = row.wrapping_add(1);
    }
    live
}
