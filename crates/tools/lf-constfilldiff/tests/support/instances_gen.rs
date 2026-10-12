// One entry per verified constant-fill rewrite, in the order of the lifted table
// in lf-core (const_fill::FILL_VALUES).
#![allow(dead_code)]

use lf_constfilldiff::rewrites;

/// One verified rewrite of the constant-fill family.
pub struct Instance {
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export. It returns zero.
    pub call: fn() -> u32,
    /// The three globals the rewrite writes, in the order it writes them.
    pub target_vas: [u32; 3],
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance {
        address: "0x00967120",
        call: || rewrites::fn_00967120::rw_00967120(),
        target_vas: [0x0106_283C, 0x0106_B6DC, 0x0110_B604],
    },
    Instance {
        address: "0x009F9600",
        call: || rewrites::fn_009F9600::rw_s18f10(),
        target_vas: [0x012B_6270, 0x012B_6274, 0x012B_6278],
    },
    Instance {
        address: "0x00B819B0",
        call: || rewrites::fn_00B819B0::rw_00B819B0(),
        target_vas: [0x0167_E2BC, 0x0167_E2C4, 0x0167_E2CC],
    },
    Instance {
        address: "0x00B819D0",
        call: || rewrites::fn_00B819D0::rw_00B819D0(),
        target_vas: [0x0167_E2B8, 0x0167_E2C0, 0x0167_E2C8],
    },
];
