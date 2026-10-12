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

pub use crate::{global, relocated};

pub mod fn_00E5F300 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f300.rs")); }
pub mod fn_00E5F340 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f340.rs")); }
pub mod fn_00E5F500 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f500.rs")); }
pub mod fn_00E5F5F0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f5f0.rs")); }
pub mod fn_00E5F610 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f610.rs")); }
pub mod fn_00E5F630 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f630.rs")); }
pub mod fn_00E5F650 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f650.rs")); }
pub mod fn_00E5F670 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f670.rs")); }
pub mod fn_00E5F690 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f690.rs")); }
pub mod fn_00E5F6B0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f6b0.rs")); }
pub mod fn_00E5F6D0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f6d0.rs")); }
pub mod fn_00E5F6F0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f6f0.rs")); }
pub mod fn_00E5F710 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f710.rs")); }
pub mod fn_00E5F730 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f730.rs")); }
pub mod fn_00E5F750 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f750.rs")); }
pub mod fn_00E5F770 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5f770.rs")); }
pub mod fn_00E5FA10 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fa10.rs")); }
pub mod fn_00E5FA30 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fa30.rs")); }
pub mod fn_00E5FA50 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fa50.rs")); }
pub mod fn_00E5FA70 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fa70.rs")); }
pub mod fn_00E5FA90 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fa90.rs")); }
pub mod fn_00E5FAB0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fab0.rs")); }
pub mod fn_00E5FB10 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fb10.rs")); }
pub mod fn_00E5FB30 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fb30.rs")); }
pub mod fn_00E5FB50 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fb50.rs")); }
pub mod fn_00E5FB70 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fb70.rs")); }
pub mod fn_00E5FB90 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fb90.rs")); }
pub mod fn_00E5FC30 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fc30.rs")); }
pub mod fn_00E5FC50 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fc50.rs")); }
pub mod fn_00E5FC70 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fc70.rs")); }
pub mod fn_00E5FC90 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fc90.rs")); }
pub mod fn_00E5FCB0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fcb0.rs")); }
pub mod fn_00E5FCD0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fcd0.rs")); }
pub mod fn_00E5FCF0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fcf0.rs")); }
pub mod fn_00E5FD10 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fd10.rs")); }
pub mod fn_00E5FD30 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fd30.rs")); }
pub mod fn_00E5FD50 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fd50.rs")); }
pub mod fn_00E5FD70 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fd70.rs")); }
pub mod fn_00E5FD90 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fd90.rs")); }
pub mod fn_00E5FDB0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fdb0.rs")); }
pub mod fn_00E5FDD0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fdd0.rs")); }
pub mod fn_00E5FE30 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e5fe30.rs")); }
pub mod fn_00E600A0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e600a0.rs")); }
pub mod fn_00E600C0 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e600c0.rs")); }
pub mod fn_00E60100 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e60100.rs")); }
pub mod fn_00E60120 { #![allow(unused_imports)] use super::{global, relocated}; use crate::export; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00e60120.rs")); }
