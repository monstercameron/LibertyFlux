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

pub mod fn_008ceb60 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008ceb60.rs"
    ));
}
pub mod fn_008ceb90 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008ceb90.rs"
    ));
}
pub mod fn_008cce00 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008cce00.rs"
    ));
}
pub mod fn_008cea60 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008cea60.rs"
    ));
}
pub mod fn_008cea30 {
    #![allow(unused_imports)]
    use super::{global, relocated};
    use crate::export;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../rewrites/verified/functions/fn_008cea30.rs"
    ));
}
