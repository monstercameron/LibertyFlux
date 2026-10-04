//! Lifted (64-bit portable) form of the pilot cluster: intrusive container
//! primitives (chain nodes, slot records, fixed-stride pools, a value list,
//! a one-shot table init).
//!
//! Rules: no raw addresses, no numbered callee slots, no pointer-width
//! dependence. Links are indices into explicit arenas; callbacks are
//! `FnMut` parameters that receive state; globals are struct fields.
//! `#![forbid(unsafe_code)]`: the whole lift is safe Rust.
#![forbid(unsafe_code)]

pub mod chain;
pub mod misc;
pub mod pool;
pub mod slots;
