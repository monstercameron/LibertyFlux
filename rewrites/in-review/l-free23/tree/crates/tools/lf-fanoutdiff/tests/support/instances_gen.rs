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
        source_va: 0x017AD148,
        target_vas: [0x01B4B304, 0x01B4B308, 0x01B4B30C],
    },
    Instance {
        address: "0x00E5BD90",
        call: || rewrites::fn_00E5BD90::rs188_00e5bd90(),
        source_va: 0x017AD148,
        target_vas: [0x01B4B2B0, 0x01B4B2B8, 0x01B4B2BC],
    },
    Instance {
        address: "0x00E5BDC0",
        call: || rewrites::fn_00E5BDC0::rs188_00e5bdc0(),
        source_va: 0x017AD148,
        target_vas: [0x01B4B310, 0x01B4B314, 0x01B4B31C],
    },
    Instance {
        address: "0x00E5BDF0",
        call: || rewrites::fn_00E5BDF0::rs188_00e5bdf0(),
        source_va: 0x017AD148,
        target_vas: [0x018D2190, 0x018D2194, 0x018D2198],
    },
    Instance {
        address: "0x00E5BEC0",
        call: || rewrites::fn_00E5BEC0::rs188_00e5bec0(),
        source_va: 0x017AD150,
        target_vas: [0x018D21A0, 0x018D21A4, 0x018D21A8],
    },
    Instance {
        address: "0x00E5BEF0",
        call: || rewrites::fn_00E5BEF0::rs188_00e5bef0(),
        source_va: 0x017AD14C,
        target_vas: [0x018D2180, 0x018D2184, 0x018D2188],
    },
    Instance {
        address: "0x00E5FE90",
        call: || rewrites::fn_00E5FE90::rw_00e5fe90(),
        source_va: 0x017AD148,
        target_vas: [0x01BB3900, 0x01BB3904, 0x01BB3908],
    },
    Instance {
        address: "0x00E622D0",
        call: || rewrites::fn_00E622D0::rw_00e622d0(),
        source_va: 0x01110CD8,
        target_vas: [0x01B4A9A0, 0x01B4A9A4, 0x01B4A9A8],
    },
];
