// Generated from the verified row-lookup rewrites: one entry per rewrite,
// in the order of the lifted table in lf-core (object_rows::ROW_LOOKUPS).
#![allow(dead_code)]

use lf_rowdiff::rewrites;

/// One verified rewrite of the row-lookup family.
pub struct Instance {
    /// The batch name of the rewrite.
    pub name: &'static str,
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export, called with the object's address and the index.
    pub call: fn(u32, u32) -> u32,
    /// Byte offset of the flag word the rewrite reads.
    pub flag_offset: usize,
    /// Byte offset of the base word the rewrite reads.
    pub base_offset: usize,
    /// Row stride the rewrite multiplies the index by.
    pub stride: u32,
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance {
        name: "fn_00BEB5C0",
        address: "0x00BEB5C0",
        call: |this, index| rewrites::fn_00BEB5C0::rw_00beb5c0(this, index),
        flag_offset: 0xc,
        base_offset: 0x44,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB5E0",
        address: "0x00BEB5E0",
        call: |this, index| rewrites::fn_00BEB5E0::rw_00beb5e0(this, index),
        flag_offset: 0xc,
        base_offset: 0x30,
        stride: 8,
    },
    Instance {
        name: "fn_00BEB600",
        address: "0x00BEB600",
        call: |this, index| rewrites::fn_00BEB600::rw_00beb600(this, index),
        flag_offset: 0xc,
        base_offset: 0x30,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB620",
        address: "0x00BEB620",
        call: |this, index| rewrites::fn_00BEB620::rw_00beb620(this, index),
        flag_offset: 0xc,
        base_offset: 0x4c,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB650",
        address: "0x00BEB650",
        call: |this, index| rewrites::fn_00BEB650::rw_00beb650(this, index),
        flag_offset: 0xc,
        base_offset: 0x20,
        stride: 24,
    },
    Instance {
        name: "fn_00BEB670",
        address: "0x00BEB670",
        call: |this, index| rewrites::fn_00BEB670::rw_00beb670(this, index),
        flag_offset: 0x8,
        base_offset: 0x40,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB690",
        address: "0x00BEB690",
        call: |this, index| rewrites::fn_00BEB690::rw_00beb690(this, index),
        flag_offset: 0x8,
        base_offset: 0x2c,
        stride: 8,
    },
    Instance {
        name: "fn_00BEB6B0",
        address: "0x00BEB6B0",
        call: |this, index| rewrites::fn_00BEB6B0::rw_00beb6b0(this, index),
        flag_offset: 0x8,
        base_offset: 0x2c,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB6D0",
        address: "0x00BEB6D0",
        call: |this, index| rewrites::fn_00BEB6D0::rw_00beb6d0(this, index),
        flag_offset: 0x8,
        base_offset: 0x48,
        stride: 20,
    },
    Instance {
        name: "fn_00BEB6F0",
        address: "0x00BEB6F0",
        call: |this, index| rewrites::fn_00BEB6F0::rw_00beb6f0(this, index),
        flag_offset: 0x8,
        base_offset: 0x1c,
        stride: 24,
    },
];
