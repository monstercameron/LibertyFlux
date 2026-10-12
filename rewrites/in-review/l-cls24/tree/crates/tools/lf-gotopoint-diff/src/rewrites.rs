//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged, one module per file, because the files
//! share top-level helper names. The include paths are relative to this
//! crate's manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

// Byte-identical verified files; their style is not linted here.
#![allow(
    unsafe_code,
    missing_docs,
    non_snake_case,
    dead_code,
    unused_doc_comments,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub mod fn_00B791F0 {
    #![allow(unused_imports)]
    use lf_checker_rt::export;
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00B791F0.rs"));
}

pub mod fn_00CBBB90 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CBBB90.rs"));
}

pub mod fn_00CB7480 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CB7480.rs"));
}

pub mod fn_00CBC000 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CBC000.rs"));
}

pub mod fn_00CBC0D0 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CBC0D0.rs"));
}

pub mod fn_00CB8AC0 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CB8AC0.rs"));
}

pub mod fn_00CAF720 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CAF720.rs"));
}

pub mod fn_00CAFC10 {
    #![allow(unused_imports)]
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00CAFC10.rs"));
}
