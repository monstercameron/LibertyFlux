//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is a module of its own, declared by its path, and is compiled
//! unchanged: it sees this crate as `lf_checker_rt` (32-bit target only). The
//! paths are relative to this file's directory, `src`, and climb to the
//! repository root (four levels in the repository; the scratch copy of this
//! crate sits four folders deeper, and its paths are written for that).

// Byte-identical verified files: they use the checker's unsafe calling
// convention and macro style, and their lint noise is not this crate's to fix.
#![allow(
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_unsafe,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_doc_comments,
    dead_code,
    missing_docs,
    non_snake_case,
    clippy::all,
    clippy::pedantic
)]

#[allow(unsafe_op_in_unsafe_fn)]
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00d76910.rs"]
pub mod fn_00d76910;

#[allow(unsafe_op_in_unsafe_fn)]
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00d76950.rs"]
pub mod fn_00d76950;

#[allow(unsafe_op_in_unsafe_fn)]
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00d77430.rs"]
pub mod fn_00d77430;

#[allow(unsafe_op_in_unsafe_fn)]
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00d77600.rs"]
pub mod fn_00d77600;
