//! The verified rewrites under test, compiled from their tracked files.
//!
//! Each file is included unchanged and sees this crate as `lf_checker_rt`
//! (32-bit target only). The include paths are relative to this crate's
//! manifest: three levels up to the repository root, then
//! `rewrites/verified/functions/`.

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

pub use crate::{global, relocated};

pub mod fn_008c6d90 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008c6d90.rs"
    ));
}
pub mod fn_00a3ce30 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00a3ce30.rs"
    ));
}
pub mod fn_00c2f620 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00c2f620.rs"
    ));
}
pub mod fn_00648b30 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00648b30.rs"
    ));
}
pub mod fn_00648c10 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_00648c10.rs"
    ));
}
pub mod fn_008731e0 {
    #![allow(unused_imports)]
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008731e0.rs"
    ));
}
