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

pub mod m_008c8860 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008c8860.rs"
    ));
}

pub mod m_00908370 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00908370.rs"
    ));
}

pub mod m_009431d0 {
    #![allow(unused_imports)]
    use crate::export;
    #[allow(unused_imports)]
    use crate::{global, relocated};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_009431d0.rs"
    ));
}
