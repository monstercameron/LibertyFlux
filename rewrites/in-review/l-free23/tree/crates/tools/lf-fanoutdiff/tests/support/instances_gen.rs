// Generated from the verified fan-out rewrites: one entry per rewrite.
#![allow(dead_code)]

use lf_fanoutdiff::rewrites;

/// One verified rewrite of the fan-out family.
pub struct Instance {
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export. It returns zero.
    pub call: fn() -> u32,
    /// The global the rewrite reads.
    pub source_va: u32,
    /// The globals the rewrite writes, in the order it writes them.
    pub target_vas: [u32; 3],
}

/// One entry per rewrite of the family.
pub const INSTANCES: &[Instance] = &[
    Instance {
        address: "0x00E5BD60",
        call: || rewrites::fn_00E5BD60::rs188_00e5bd60(),
        source_va: 0x17ad148,
        target_vas: [0x1b4b304, 0x1b4b308, 0x1b4b30c],
    },
    Instance {
        address: "0x00E5BD90",
        call: || rewrites::fn_00E5BD90::rs188_00e5bd90(),
        source_va: 0x17ad148,
        target_vas: [0x1b4b2b0, 0x1b4b2b8, 0x1b4b2bc],
    },
    Instance {
        address: "0x00E5BDC0",
        call: || rewrites::fn_00E5BDC0::rs188_00e5bdc0(),
        source_va: 0x17ad148,
        target_vas: [0x1b4b310, 0x1b4b314, 0x1b4b31c],
    },
    Instance {
        address: "0x00E5BDF0",
        call: || rewrites::fn_00E5BDF0::rs188_00e5bdf0(),
        source_va: 0x17ad148,
        target_vas: [0x18d2190, 0x18d2194, 0x18d2198],
    },
    Instance {
        address: "0x00E5BEC0",
        call: || rewrites::fn_00E5BEC0::rs188_00e5bec0(),
        source_va: 0x17ad150,
        target_vas: [0x18d21a0, 0x18d21a4, 0x18d21a8],
    },
    Instance {
        address: "0x00E5BEF0",
        call: || rewrites::fn_00E5BEF0::rs188_00e5bef0(),
        source_va: 0x17ad14c,
        target_vas: [0x18d2180, 0x18d2184, 0x18d2188],
    },
];
