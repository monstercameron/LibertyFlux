//! The verified slot-record store rewrites under test, read from their tracked files.
//!
//! Each file is read unchanged and sees this crate as lf_checker_rt (32-bit
//! target only).

// Byte-identical verified files; their style is not linted here.
#![allow(unsafe_code, missing_docs, non_snake_case, unused_doc_comments, unused_imports, unused_mut, unused_variables, unused_unsafe, dead_code, clippy::all, clippy::pedantic)]

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00897ef0.rs"]
pub mod fn_00897EF0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00897fb0.rs"]
pub mod fn_00897FB0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00898060.rs"]
pub mod fn_00898060;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_008980d0.rs"]
pub mod fn_008980D0;

