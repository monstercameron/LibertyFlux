//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: six levels up to the repository root, then
//! `rewrites/verified/functions/` (this proof crate lives in the lane's
//! scratch tree; the coordinator re-points the depth on integration).

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::callee_addr;
pub use crate::global;
pub use crate::relocated;

include!("rewrites_gen.rs");
