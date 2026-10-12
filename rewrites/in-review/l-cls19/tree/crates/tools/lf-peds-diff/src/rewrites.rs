//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt` (32-bit
//! target only). The include paths are relative to this crate's manifest: three
//! levels up to the repository root, then `rewrites/verified/functions/`.
//!
//! One module per rewrite: each file's top-level names stay in its own scope.

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

/// Deleting destructor of the scenario (the scenario's first virtual method).
pub mod fn_00c7a880 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c7a880.rs"
    ));
}

/// Clone factory of the scenario (the scenario's second virtual method).
pub mod fn_00c7b220 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c7b220.rs"
    ));
}

/// Blend weight of the scenario (the scenario's seventh virtual method).
pub mod fn_00c7fc70 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c7fc70.rs"
    ));
}

/// Run tick of the scenario (the scenario's sixth virtual method).
pub mod fn_00c819e0 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c819e0.rs"
    ));
}
