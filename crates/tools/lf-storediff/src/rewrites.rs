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
    unused_variables,
    clippy::all,
    clippy::pedantic
)]

pub use crate::global;

pub mod fn_006A33F0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_006a33f0.rs"
    ));
}
pub mod fn_008F9260 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008f9260.rs"
    ));
}
pub mod fn_009BBA00 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_009bba00.rs"
    ));
}
pub mod fn_00B02D10 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b02d10.rs"
    ));
}
pub mod fn_00DB1920 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00db1920.rs"
    ));
}
pub mod fn_00E607C0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e607c0.rs"
    ));
}
pub mod fn_00E60880 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e60880.rs"
    ));
}
pub mod fn_00E60E50 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e60e50.rs"
    ));
}
