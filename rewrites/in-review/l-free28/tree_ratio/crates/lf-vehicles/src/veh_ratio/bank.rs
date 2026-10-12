//! The verified ratio set, owned as a bank of cells.

use crate::veh_ratio::RatioCell;

/// Every verified ratio coefficient in registry order.
///
/// The original invokes the family through a table of code pointers;
/// the bank is that set as owned data: one [`RatioCell`] per verified
/// instance, at the index [`registry::ROWS`](crate::veh_ratio::registry::ROWS)
/// gives it. [`RatioBank::refresh_all`] recomputes every quotient, and
/// [`RatioBank::refresh`] recomputes one.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RatioBank {
    /// The cells, in registry order.
    cells: Vec<RatioCell>,
}

impl RatioBank {
    /// Builds a bank from its cells in registry order.
    #[must_use]
    pub fn new(cells: Vec<RatioCell>) -> Self {
        Self { cells }
    }

    /// The number of cells.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Whether the bank holds no cells.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// The cell at `index`, or `None` past the end.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&RatioCell> {
        self.cells.get(index)
    }

    /// Recomputes the quotient of the cell at `index`.
    ///
    /// The original has no indexed call: each instance names its own
    /// three globals. The index is new, and names the cell by its row in
    /// [`registry::ROWS`](crate::veh_ratio::registry::ROWS).
    ///
    /// # Panics
    ///
    /// When `index` is past the end of the bank.
    pub fn refresh(&mut self, index: usize) {
        self.cells[index].refresh();
    }

    /// Recomputes every quotient, in registry order.
    ///
    /// This is [`RatioCell::refresh`] run over the whole bank: the
    /// per-cell behaviour is proven against each verified rewrite, and
    /// the walk itself is covered by the host tests.
    pub fn refresh_all(&mut self) {
        for cell in &mut self.cells {
            cell.refresh();
        }
    }
}
