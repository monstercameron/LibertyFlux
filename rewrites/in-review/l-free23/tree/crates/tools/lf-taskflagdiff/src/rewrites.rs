//! The verified task-id flag-bit rewrites under test, read from their tracked files.
//!
//! Each file is read unchanged and sees this crate as `lf_checker_rt` (32-bit
//! target only).

// Byte-identical verified files; their style is not linted here.
#![allow(unsafe_code, missing_docs, non_snake_case, unused_doc_comments, unused_imports, unused_mut, unused_variables, unused_unsafe, dead_code, clippy::all, clippy::pedantic)]

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00c50df0.rs"]
pub mod fn_00C50DF0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00c50e10.rs"]
pub mod fn_00C50E10;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00c50e30.rs"]
pub mod fn_00C50E30;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[path = "../../../../../../../../rewrites/verified/functions/fn_00c50e50.rs"]
pub mod fn_00C50E50;

