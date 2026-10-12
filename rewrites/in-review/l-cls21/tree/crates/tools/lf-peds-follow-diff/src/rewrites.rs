//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged, as a module of its own (the files share
//! top-level helper names). The include paths are relative to this crate's
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
    unused_variables,
    unused_unsafe,
    clippy::all,
    clippy::pedantic
)]

/// Verified rewrite: route cursor copied into an output record.
pub mod fn_00cbc9f0 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cbc9f0.rs"
    ));
}

/// Verified rewrite: current route point copied out (twelve bytes).
pub mod fn_00cb73d0 {
    use lf_checker_rt::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cb73d0.rs"
    ));
}

/// Verified rewrite: waypoint at an index copied out (twelve bytes).
pub mod fn_00cb73a0 {
    use lf_checker_rt::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cb73a0.rs"
    ));
}

/// Verified rewrite: waypoint pick and order.
pub mod fn_00cb2970 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cb2970.rs"
    ));
}

/// Verified rewrite: advance after a finished sub-task.
pub mod fn_00cb3b50 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cb3b50.rs"
    ));
}


