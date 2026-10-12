//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`. The include paths are relative to
//! this crate's manifest: three levels up to the repository root, then `rewrites/verified/functions/`.

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

pub mod m_00937b60 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00937b60.rs"
    ));
}

pub mod m_00a09f20 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00a09f20.rs"
    ));
}

pub mod m_00c886c0 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c886c0.rs"
    ));
}
