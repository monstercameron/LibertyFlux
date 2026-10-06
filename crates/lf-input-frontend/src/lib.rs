//! `lf-input-frontend`: input mapping, menus and the front end.
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

pub mod font_string;
pub mod input_slot;
pub mod layout;
pub mod ui_clip;
