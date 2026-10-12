//! Lifted replay progress bar: the object's teardown, its deleting
//! destructor and its per-frame update, as ordinary Rust.
//!
//! The original's progress bar is a class with a virtual interface. Of
//! its eight named methods, three are lifted here: the deleting
//! destructor, the update, and the teardown routine that the deleting
//! destructor runs first (the original names it with a constructor-style
//! name). The other four are not verified and are not lifted (see
//! [`registry`]).
//!
//! [`ReplayProgressBar`] owns the object's words as named fields. Its
//! collaborators travel as opaque [`Handle32`] cookies, and every call to
//! them is one method of [`ReplayBarWorld`]. The shared globals the update
//! reads and writes are passed in as [`ReplayBarGlobals`]. Nothing here
//! has an address, a virtual slot number, a pointer width or `unsafe`.
//!
//! Proof is differential: each lifted method runs against its verified
//! 32-bit rewrite on the same generated inputs, comparing results, every
//! written word of the object and the globals, and every collaborator call
//! in order with its arguments (see the `lf-replaybar-diff` test crate).
//! Nothing here is verified by the checker itself.
//!
//! [`Handle32`]: lf_core::Handle32

#![forbid(unsafe_code)]

mod bar;
mod world;

pub mod registry;

pub use bar::{OWNED_SLOTS, ReplayBarGlobals, ReplayProgressBar, UpdateOutcome};
pub use world::{BarTag, InnerTag, OwnedTag, Part, ReplayBarWorld};
