//! The verified fan-out rewrites under test, read from their tracked files.
//!
//! Each file is read unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only).

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    dead_code,
    clippy::all,
    clippy::pedantic
)]

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bd60.rs"]
pub mod fn_00E5BD60;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bd90.rs"]
pub mod fn_00E5BD90;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bdc0.rs"]
pub mod fn_00E5BDC0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bdf0.rs"]
pub mod fn_00E5BDF0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bec0.rs"]
pub mod fn_00E5BEC0;

// The verified rewrite file, read unchanged. Four levels up from this
// source folder to the repository root, then rewrites/verified/functions/.
#[rustfmt::skip]
#[path = "../../../../rewrites/verified/functions/fn_00e5bef0.rs"]
pub mod fn_00E5BEF0;

pub mod fn_00E5FE90 {
    use crate::{export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5fe90.rs"
    ));
}

pub mod fn_00E622D0 {
    use crate::{export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e622d0.rs"
    ));
}
