//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`. The include paths are
//! relative to this crate's manifest: three levels up to the repository root, then
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

pub mod m_0098c170 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_0098c170.rs"
    ));
}

pub mod m_009fbac0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_009fbac0.rs"
    ));
}

pub mod m_00985d60 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00985d60.rs"
    ));
}

pub mod m_00b4bad0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00b4bad0.rs"
    ));
}

pub mod m_00942840 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00942840.rs"
    ));
}

pub mod m_009ccd80 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_009ccd80.rs"
    ));
}
