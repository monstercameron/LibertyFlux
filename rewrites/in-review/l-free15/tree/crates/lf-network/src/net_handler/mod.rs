//! The network handler slots: one shared slot per group, one save cell
//! per unit.
//!
//! Twelve verified `net_handler_swap_*` routines share one body (proved by
//! hashing their normalised bodies, not by eye): read the shared slot,
//! park the old handler in the unit's save cell, install the unit's
//! replacement, and answer the old handler. Eleven instances share one
//! slot, one has a slot of its own, and every instance has its own save
//! cell and replacement. The lift owns that shape as [`HandlerSlots`]
//! with one generic [`swap`](HandlerSlots::swap) method, proven once per
//! instance against its verified rewrite.
//!
//! Handler routines live in code not yet lifted, so slots carry them as
//! opaque [`Handler`] cookies: compared, copied and handed back, nothing
//! else. Zero travels as `None`.

pub mod registry;
pub mod slot;

pub use slot::{Handler, HandlerSlots, HandlerTag};
