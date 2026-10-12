//! `lf-peds-tasks`: pedestrians and the task system.
//!
//! README for future lanes:
//! - Portable rewrite code for this subsystem lives in this crate root and
//!   is free to be idiomatic Rust (pointer-width independent from the
//!   first line, per plan.md).
//! - Fixed 32-bit memory layouts live in [`layout`]: `#[repr(C)]` types
//!   only, pointers as `lf_core::Ptr32`, every type carrying
//!   `lf_core::assert_size!` checks and every depended-on field carrying
//!   `lf_core::assert_offset!` checks.
//! - No `unsafe` without re-allowing it on the enclosing module with a
//!   justification comment (workspace lints deny it otherwise).
//! - Platform calls (window, files, threads, time) go through
//!   `lf-platform`; `cfg(target_os)` appears nowhere here.
//! - Never original game code here: structures, symbols and new Rust only.

pub mod event_handler;
pub mod follow_point_route;
pub mod go_to_point_on_route;
pub mod layout;
pub mod melee;
pub mod move_between_points;
pub mod ped_task;
pub mod peds_task;
pub mod seek_cover;
pub mod task_ctor;
pub mod tasks;
