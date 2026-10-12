//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables,
    dead_code,
    clippy::all,
    clippy::pedantic
)]

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rewrites/verified/functions/fn_00dd46c0.rs"
));

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rewrites/verified/functions/fn_00dd3040.rs"
));

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rewrites/verified/functions/fn_00dd37e0.rs"
));
