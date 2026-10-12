//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged, one module per file, because the files
//! share top-level helper names. The include paths are relative to this
//! crate's manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.
//!
//! The constructor file qualifies its runtime names; the four virtual-method
//! files use the bare `export!` macro with `callee_thiscall!` and `global!`,
//! which each module imports.

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

pub mod fn_00ccefb0 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00ccefb0.rs"
    ));
}

pub mod fn_00ccf840 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_thiscall, export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00ccf840.rs"
    ));
}

pub mod fn_00cd0450 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_thiscall, export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cd0450.rs"
    ));
}

pub mod fn_00cd0d00 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_thiscall, export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cd0d00.rs"
    ));
}

pub mod fn_00cd0d40 {
    #![allow(unused_imports)]
    use lf_checker_rt::{callee_thiscall, export, global};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00cd0d40.rs"
    ));
}
