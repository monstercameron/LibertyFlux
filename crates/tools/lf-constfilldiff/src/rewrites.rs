//! The verified constant-fill rewrites under test, included from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt` (or
//! `lf_k2_rt`, the name one of the files imports) on the 32-bit target only.
//! The include paths are relative to this crate's manifest: three levels up to
//! the repository root, then `rewrites/verified/functions/`.

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

pub mod fn_00967120 {
    #![allow(unused_imports)]
    use crate::{export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00967120.rs"
    ));
}

pub mod fn_009F9600 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_009f9600.rs"
    ));
}

pub mod fn_00B819B0 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b819b0.rs"
    ));
}

pub mod fn_00B819D0 {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b819d0.rs"
    ));
}
