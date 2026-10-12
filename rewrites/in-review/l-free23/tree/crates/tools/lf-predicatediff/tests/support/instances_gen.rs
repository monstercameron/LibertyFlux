// One entry per verified field predicate, in the order of the lifted table in
// lf-core (field_predicate::PREDICATES).
#![allow(dead_code)]

use lf_predicatediff::rewrites;

/// One verified rewrite of the field-predicate family.
pub struct Instance {
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export: the object's address, returns zero or one.
    pub call: fn(u32) -> u32,
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance { address: "0x0099F630", call: |this| rewrites::fn_0099F630::rw_0099f630(this) },
    Instance { address: "0x0099F6E0", call: |this| rewrites::fn_0099F6E0::rw_0099f6e0(this) },
    Instance { address: "0x0096BCE0", call: |this| rewrites::fn_0096BCE0::rw_0096bce0(this) },
    Instance { address: "0x00A29D30", call: |this| rewrites::fn_00A29D30::rw_00a29d30(this) },
    Instance { address: "0x008D8EF0", call: |this| rewrites::fn_008D8EF0::rw_008d8ef0(this) },
    Instance { address: "0x00CA28F0", call: |this| rewrites::fn_00CA28F0::rw_00ca28f0(this) },
];
