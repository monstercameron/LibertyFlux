//! Row tables: guarded counts, cell reads and the cell iterator.
//!
//! Lifted from three verified pure routines (no class) over the 160-byte
//! row tables: the guarded row-count read, the two-level cell read, and
//! the (row, cell) cursor iterator. The lift owns the rows and their
//! cells; the row blocks stay at the boundary.

/// One row: its cells in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The cell words.
    pub cells: Vec<u32>,
}

/// A table of rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowTable {
    /// The rows in order.
    pub rows: Vec<Row>,
}

impl RowTable {
    /// Reads row `row`'s 16-bit cell count when `enabled` (the enable byte
    /// at +0x73), else 0.
    ///
    /// # Panics
    ///
    /// When `row` is past the table, or the row holds more than
    /// `u16::MAX` cells.
    #[must_use]
    pub fn count_guarded(&self, enabled: bool, row: u32) -> u16 {
        if !enabled {
            return 0;
        }
        let cells = self
            .rows
            .get(row as usize)
            .unwrap_or_else(|| panic!("row {row} past {} rows", self.rows.len()))
            .cells
            .len();
        u16::try_from(cells).expect("cell counts past u16::MAX are out of domain")
    }

    /// Reads cell `cell` of row `row`.
    ///
    /// # Panics
    ///
    /// When either index is past its table (the original reads on).
    #[must_use]
    pub fn cell(&self, row: u32, cell: u32) -> u32 {
        let cells = &self
            .rows
            .get(row as usize)
            .unwrap_or_else(|| panic!("row {row} past {} rows", self.rows.len()))
            .cells;
        *cells.get(cell as usize).unwrap_or_else(|| {
            panic!(
                "cell {cell} past {} cells of row {row}",
                cells.len()
            )
        })
    }

    /// A cursor over the table, starting at (`row`, `cell`).
    #[must_use]
    pub fn cursor(&self, row: i32, cell: u32) -> RowCursor<'_> {
        RowCursor { table: self, row, cell }
    }

    /// Row count for the cursor's end checks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether the table holds no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// The (row, cell) cursor the iterator steps.
#[derive(Debug, Clone, Copy)]
pub struct RowCursor<'a> {
    /// The table walked.
    table: &'a RowTable,
    /// Current row (the 32-bit form keeps it signed).
    pub row: i32,
    /// Current cell.
    pub cell: u32,
}

impl RowCursor<'_> {
    /// Steps the cursor and fetches the current cell, writing it through
    /// `out` and answering true. Answers false when the rows run out,
    /// leaving `out` untouched. The cell advances first; when it reaches
    /// the row's count (signed) the row advances with the cell reset,
    /// skipping empty rows.
    ///
    /// # Panics
    ///
    /// When the cursor starts on a negative row, when a row holds more
    /// than `u16::MAX` cells, or when the table holds more than
    /// `i32::MAX` rows.
    pub fn step(&mut self, out: &mut u32) -> bool {
        let rows = i32::try_from(self.table.len())
            .expect("row tables past i32::MAX rows are out of domain");
        let mut row = self.row;
        assert!(
            row >= 0,
            "cursor starts on negative row {row}: the original reads before the table"
        );
        if row >= rows {
            return false;
        }
        loop {
            let cell = self.cell.wrapping_add(1);
            self.cell = cell;
            let limit = self.table.rows[row as usize].cells.len();
            let limit16 =
                u16::try_from(limit).expect("cell counts past u16::MAX are out of domain");
            if (cell as i32) < i32::from(limit16) {
                break;
            }
            row = row.wrapping_add(1);
            self.row = row;
            self.cell = 0;
            if row >= rows {
                return false;
            }
            assert!(
                row >= 0,
                "cursor wrapped past row {row}: the original reads before the table"
            );
            let first = self.table.rows[row as usize].cells.len();
            let first16 =
                u16::try_from(first).expect("cell counts past u16::MAX are out of domain");
            if 0u16 >= first16 {
                continue;
            }
            break;
        }
        let cells = &self.table.rows[self.row as usize].cells;
        let at = usize::try_from(self.cell)
            .ok()
            .and_then(|at| cells.get(at).map(|_| at))
            .unwrap_or_else(|| {
                panic!(
                    "stepped cell {} past {} cells of row {}",
                    self.cell,
                    cells.len(),
                    self.row
                )
            });
        *out = cells[at];
        true
    }
}
