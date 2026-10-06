//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: seven levels up to the repository root in the lane tree,
//! then `rewrites/verified/functions/`. NOTE for the coordinator: when
//! this crate is integrated at `crates/tools/`, the depth becomes three
//! (`/../../../rewrites/...`).

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::callee_addr;
pub use crate::global;
pub use crate::relocated;

include!("rewrites_gen.rs");
