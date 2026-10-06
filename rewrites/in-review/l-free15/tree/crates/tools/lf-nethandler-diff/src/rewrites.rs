//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged (32-bit target only). The include paths
//! are relative to this crate's manifest: three levels up to the
//! repository root, then `rewrites/verified/functions/`.

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

pub use crate::global;
pub use crate::relocated;

include!("rewrites_gen.rs");
