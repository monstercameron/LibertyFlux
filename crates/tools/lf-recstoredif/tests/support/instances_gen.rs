// One entry per verified slot-record store, in the order of the lifted table
// in lf-core (slot_store::SLOT_STORES).
#![allow(dead_code)]

use lf_recstoredif::rewrites;

/// One verified rewrite of the slot-record store family.
pub struct Instance {
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export: the object's address and the value.
    pub call: fn(u32, u32),
    /// Byte offset of the field within the slot record.
    pub field_offset: u32,
    /// Width of the field in bytes.
    pub width: usize,
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance {
        address: "0x00897FB0",
        call: |this, value| rewrites::fn_00897FB0::rw_00897fb0(this, value),
        field_offset: 0x5a,
        width: 2,
    },
    Instance {
        address: "0x00898060",
        call: |this, value| rewrites::fn_00898060::rw_00898060(this, value),
        field_offset: 0x58,
        width: 2,
    },
    Instance {
        address: "0x008980D0",
        call: |this, value| rewrites::fn_008980D0::rw_008980d0(this, value),
        field_offset: 0x52,
        width: 2,
    },
    Instance {
        address: "0x00897EF0",
        call: |this, value| rewrites::fn_00897EF0::rw_00897ef0(this, value),
        field_offset: 0x5d,
        width: 1,
    },
];
