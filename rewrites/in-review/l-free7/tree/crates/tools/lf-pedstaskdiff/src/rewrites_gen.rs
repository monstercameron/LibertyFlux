// One module per verified rewrite: files share top-level helper names,
// so they cannot live in one scope. All four use the qualified
// `lf_checker_rt::` dialect. Paths climb seven levels from this scratch
// crate's manifest directory to the repository root; at integration the
// coordinator shortens them to three (`crates/tools/lf-pedstaskdiff`).
pub mod fn_00D44AB0 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00d44ab0.rs")); }
pub mod fn_00BE4F60 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00be4f60.rs")); }
pub mod fn_00CB8020 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00cb8020.rs")); }
pub mod fn_00CB7CF0 { #![allow(unused_imports)] use super::{callee_addr, global, relocated}; include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../rewrites/verified/functions/fn_00cb7cf0.rs")); }
