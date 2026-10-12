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

pub mod m_00ab6ce0 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00ab6ce0.rs"
    ));
}

pub mod m_00ac8b60 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00ac8b60.rs"
    ));
}

pub mod m_00e61630 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../../../../rewrites/verified/functions/fn_00e61630.rs"
    ));
}
