// Generated from the verified rewrites: one entry per constant-store instance.
#![allow(dead_code)]

use lf_storediff::rewrites;

/// One verified rewrite: the two words it stores and the constants.
pub struct Instance {
    /// The batch name of the rewrite.
    pub name: &'static str,
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export, called in the test runtime.
    pub call: fn() -> u32,
    /// The constant the rewrite returns, or none when it returns nothing.
    pub returns: Option<u32>,
    /// The word stored first.
    pub first: u32,
    /// The constant stored into the first word.
    pub first_value: u32,
    /// The word stored second.
    pub second: u32,
    /// The constant stored into the second word.
    pub second_value: u32,
}

/// Every word the batch touches.
pub const WORDS: &[u32] = &[
    0x018DD968, 0x018DDD78, 0x0118E7D4, 0x0118E7D8, 0x0128E944, 0x0128E948, 0x016010A4, 0x016010A8,
    0x017A65A8, 0x017A65AC, 0x019F3880, 0x019F3884, 0x019F393C, 0x019F3940, 0x019FD9B0, 0x019FD9B4,
];

/// One entry per instance.
pub const INSTANCES: &[Instance] = &[
    Instance {
        name: "fn_006A33F0",
        address: "0x006A33F0",
        call: || {
            rewrites::fn_006A33F0::rw_006a33f0();
            0
        },
        returns: None,
        first: 0x018DD968,
        first_value: 0x00000000,
        second: 0x018DDD78,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_008F9260",
        address: "0x008F9260",
        call: || rewrites::fn_008F9260::rw_008f9260(),
        returns: Some(0),
        first: 0x0118E7D4,
        first_value: 0x00000000,
        second: 0x0118E7D8,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_009BBA00",
        address: "0x009BBA00",
        call: || rewrites::fn_009BBA00::rw_009bba00(),
        returns: Some(0),
        first: 0x0128E944,
        first_value: 0x00000000,
        second: 0x0128E948,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_00B02D10",
        address: "0x00B02D10",
        call: || rewrites::fn_00B02D10::rw_00b02d10(),
        returns: Some(0),
        first: 0x016010A4,
        first_value: 0x00000000,
        second: 0x016010A8,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_00DB1920",
        address: "0x00DB1920",
        call: || rewrites::fn_00DB1920::rw_00db1920(),
        returns: Some(0),
        first: 0x017A65A8,
        first_value: 0x00000003,
        second: 0x017A65AC,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_00E607C0",
        address: "0x00E607C0",
        call: || rewrites::fn_00E607C0::rw_00e607c0(),
        returns: Some(0),
        first: 0x019F3880,
        first_value: 0x00000000,
        second: 0x019F3884,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_00E60880",
        address: "0x00E60880",
        call: || rewrites::fn_00E60880::rw_00e60880(),
        returns: Some(0),
        first: 0x019F393C,
        first_value: 0x00000000,
        second: 0x019F3940,
        second_value: 0x00000000,
    },
    Instance {
        name: "fn_00E60E50",
        address: "0x00E60E50",
        call: || rewrites::fn_00E60E50::rw_00e60e50(),
        returns: Some(0),
        first: 0x019FD9B0,
        first_value: 0x00000000,
        second: 0x019FD9B4,
        second_value: 0x00000000,
    },
];
