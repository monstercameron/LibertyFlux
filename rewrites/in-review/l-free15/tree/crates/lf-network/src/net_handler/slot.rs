//! One shared handler slot with per-unit save cells.
//!
//! Lifted from the twelve verified `net_handler_swap_*` instances: one
//! routine proved twelve times, over each instance's shared slot, save
//! cell and replacement handler. The 32-bit form keeps the shared slot
//! and each save cell as one global word, and installs the replacement
//! as a relocated image address. The lift owns all `N + 1` words as
//! [`Option<Handler>`] fields, so every word the original can hold is
//! representable and no path narrows.

use lf_core::boundary::Handle32;

/// Identity of a network handler routine: the slots' word meaning.
///
/// Handler routines live in code not yet lifted, so they travel as
/// opaque handles until their owner lifts.
#[derive(Debug)]
pub struct HandlerTag;

/// An opaque handler identity: the word the slots carry.
pub type Handler = Handle32<HandlerTag>;

/// One shared handler slot with `N` per-unit save cells.
///
/// Eleven swap instances share one slot behind eleven save cells; the
/// twelfth instance has a slot of its own behind one save cell. Both are
/// this type with different `N`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlerSlots<const N: usize> {
    /// The installed handler (`None` is the original's zero).
    current: Option<Handler>,
    /// One save cell per unit, holding whatever each unit last displaced.
    saves: [Option<Handler>; N],
}

impl<const N: usize> HandlerSlots<N> {
    /// Empty slots: no handler installed, every save cell clear.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            current: None,
            saves: [None; N],
        }
    }

    /// Parks the installed handler in `unit`'s save cell, installs `new`,
    /// and answers the displaced handler.
    ///
    /// The 32-bit form reads the shared slot, writes the old handler to
    /// the unit's save cell, overwrites the shared slot with the unit's
    /// replacement, and returns the old handler. Every other save cell is
    /// untouched.
    ///
    /// # Panics
    ///
    /// When `unit` is not one of the `N` save cells. The original has no
    /// such call: each instance names its own cell.
    pub fn swap(&mut self, unit: usize, new: Option<Handler>) -> Option<Handler> {
        let old = core::mem::replace(&mut self.current, new);
        self.saves[unit] = old;
        old
    }

    /// The installed handler.
    #[must_use]
    pub fn current(&self) -> Option<Handler> {
        self.current
    }

    /// What `unit`'s save cell holds.
    ///
    /// # Panics
    ///
    /// When `unit` is not one of the `N` save cells.
    #[must_use]
    pub fn saved(&self, unit: usize) -> Option<Handler> {
        self.saves[unit]
    }

}
