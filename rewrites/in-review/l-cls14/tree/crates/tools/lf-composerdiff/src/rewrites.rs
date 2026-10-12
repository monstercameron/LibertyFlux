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
    unused_unsafe,
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

pub mod fn_00878180 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878180.rs"));
}
pub mod fn_008781B0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008781b0.rs"));
}
pub mod fn_008781D0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008781d0.rs"));
}
pub mod fn_00878290 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878290.rs"));
}
pub mod fn_008782C0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008782c0.rs"));
}
pub mod fn_008782E0 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_008782e0.rs"));
}

pub mod fn_00877860 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00877860.rs"));
}
pub mod fn_00878250 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878250.rs"));
}

pub mod fn_00878200 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878200.rs"));
}

pub mod fn_00877890 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00877890.rs"));
}
pub mod fn_00879030 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00879030.rs"));
}

pub mod fn_00878D90 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878d90.rs"));
}

pub mod fn_00878E00 {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00878e00.rs"));
}
