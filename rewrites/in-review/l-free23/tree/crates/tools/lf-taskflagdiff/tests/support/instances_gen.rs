// One entry per verified flag-bit rewrite, in the order of the lifted table in
// lf-core (task_flags::TASK_FLAG_BITS).
#![allow(dead_code)]

use lf_taskflagdiff::rewrites;

/// One verified rewrite of the task-id flag-bit family.
pub struct Instance {
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export: the record's address, returns the bit.
    pub call: fn(u32) -> u32,
    /// The bit the rewrite returns.
    pub bit: u32,
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance {
        address: "0x00C50DF0",
        call: |record| rewrites::fn_00C50DF0::rw_00c50df0(record),
        bit: 26,
    },
    Instance {
        address: "0x00C50E10",
        call: |record| rewrites::fn_00C50E10::rw_00c50e10(record),
        bit: 14,
    },
    Instance {
        address: "0x00C50E30",
        call: |record| rewrites::fn_00C50E30::rw_00c50e30(record),
        bit: 15,
    },
    Instance {
        address: "0x00C50E50",
        call: |record| rewrites::fn_00C50E50::rw_00c50e50(record),
        bit: 25,
    },
];
