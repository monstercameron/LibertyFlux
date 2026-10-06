//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths climb from this crate's
//! manifest to the repository root, then `rewrites/verified/functions/`:
//! seven levels in the lane's scratch tree (three once integrated).

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    dead_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::callee_addr;
pub use crate::global;
pub use crate::relocated;

include!("rewrites_gen.rs");
