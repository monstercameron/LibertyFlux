//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include path below is relative to this
//! manifest: `../../../../rewrites/...` from the lane's `tree/` copy,
//! `../../../rewrites/...` once integrated into the repository.

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

use crate::callee_addr;
use crate::callee_fastcall;
use crate::callee_thiscall;
use crate::export;
use crate::relocated;

/// Includes verified rewrite files by their base name.
macro_rules! verified {
    ($($file:literal),* $(,)?) => {
        $(include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../rewrites/verified/functions/",
            $file,
            ".rs"
        ));)*
    };
}

include!("rewrites_gen.rs");
