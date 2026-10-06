//! `lf-peds-tasks` (lane l-free4 scratch copy): pedestrians and the task system.
//!
//! Scratch layout for review: only [`layout`] (verbatim copy) and the new
//! [`ped_task`] module. The coordinator integrates `ped_task/` into the
//! tracked crate, whose `lib.rs` gains one `pub mod ped_task;` line.

pub mod layout;
pub mod ped_task;
