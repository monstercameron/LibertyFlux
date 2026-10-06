// One module per verified rewrite: files share top-level helper names,
// so they cannot live in one scope. Files using the bare `export!` macro
// import it from the crate root; the rest qualify it.
// Scratch-tree include depth (seven levels to the repository root); on
// integration the coordinator repoints these at `/../../../rewrites/...`.
pub mod fn_0088AC00 { #![allow(unused_imports)] use crate::{export}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_0088ac00.rs")); }
pub mod fn_009A3600 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_009a3600.rs")); }
pub mod fn_009A38A0 { #![allow(unused_imports)] use crate::{export}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_009a38a0.rs")); }
pub mod fn_00D8C510 { #![allow(unused_imports)] use crate::{export}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00d8c510.rs")); }
pub mod fn_00D8C570 { #![allow(unused_imports)] use crate::{export}; use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00d8c570.rs")); }
