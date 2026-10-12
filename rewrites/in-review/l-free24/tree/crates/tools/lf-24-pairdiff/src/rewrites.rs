//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`. The include paths climb
//! three levels up to the repository root, then `rewrites/verified/functions/`.

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

pub mod m_006036b0 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_006036b0.rs"
    ));
}

pub mod m_00891540 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00891540.rs"
    ));
}

pub mod m_008b1080 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_008b1080.rs"
    ));
}

pub mod m_00b4fdc0 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00b4fdc0.rs"
    ));
}

pub mod m_00c08c40 {
    #![allow(unused_imports)]
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00c08c40.rs"
    ));
}
