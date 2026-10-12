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

pub mod m_00e5be20 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5be20.rs"
    ));
}

pub mod m_00e5be40 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5be40.rs"
    ));
}

pub mod m_00e5be60 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5be60.rs"
    ));
}

pub mod m_00e5be80 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5be80.rs"
    ));
}

pub mod m_00e5fec0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e5fec0.rs"
    ));
}

pub mod m_00e66fc0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e66fc0.rs"
    ));
}

pub mod m_00e67010 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e67010.rs"
    ));
}

pub mod m_00e670c0 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e670c0.rs"
    ));
}

pub mod m_00e6af50 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e6af50.rs"
    ));
}

pub mod m_00e6c750 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e6c750.rs"
    ));
}

pub mod m_00e6c890 {
    #![allow(unused_imports)]
    use super::global;
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00e6c890.rs"
    ));
}
