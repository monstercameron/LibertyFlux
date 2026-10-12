//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged. Its `lf_checker_rt` paths resolve to
//! the shared runtime crate this test crate depends on. The include paths
//! are relative to this crate's manifest: three levels up to the repository
//! root, then `rewrites/verified/functions/`.

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

pub mod fn_00878640 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878640.rs"));
}
pub mod fn_00878670 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878670.rs"));
}
pub mod fn_008786E0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008786e0.rs"));
}
pub mod fn_008700E0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008700e0.rs"));
}
pub mod fn_00870180 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00870180.rs"));
}
