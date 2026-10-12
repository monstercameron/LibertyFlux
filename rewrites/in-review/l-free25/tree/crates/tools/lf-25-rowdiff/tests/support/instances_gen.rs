//! The instances of the static-row family, generated from the verified rewrites.
//!
//! Each entry names one rewrite: its image address, the first word of the block
//! it writes, its two constants as the rewrite computes them, and the return
//! its contract compares (`None` when the contract has no return).

use lf_25_rowdiff::rewrites;

/// One verified rewrite of the family.
#[allow(dead_code)]
pub struct Instance {
    /// The export name of the rewrite.
    pub name: &'static str,
    /// The image address of the rewrite, for the report.
    pub address: u32,
    /// The image address of the first word of the block it writes.
    pub base: u32,
    /// The constant it writes into the first word.
    pub first: u32,
    /// The constant it writes into the fourth word.
    pub second: u32,
    /// The return the rewrite's contract compares, if any.
    pub returns: Option<u32>,
    /// Calls the rewrite; a rewrite with no return gives zero here, unused.
    pub call: fn() -> u32,
}

fn call_fn_00e5c7e0() -> u32 {
    { rewrites::fn_00e5c7e0::rw_00e5c7e0(); 0 }
}

fn call_fn_00e5c820() -> u32 {
    { rewrites::fn_00e5c820::rw_00e5c820(); 0 }
}

fn call_fn_00e5c860() -> u32 {
    { rewrites::fn_00e5c860::rw_00e5c860(); 0 }
}

fn call_fn_00e5c8e0() -> u32 {
    { rewrites::fn_00e5c8e0::rw_00e5c8e0(); 0 }
}

fn call_fn_00e5c920() -> u32 {
    { rewrites::fn_00e5c920::rw_00e5c920(); 0 }
}

fn call_fn_00e5c960() -> u32 {
    { rewrites::fn_00e5c960::rw_00e5c960(); 0 }
}

fn call_fn_00e5c9a0() -> u32 {
    { rewrites::fn_00e5c9a0::rw_00e5c9a0(); 0 }
}

fn call_fn_00e5c9e0() -> u32 {
    { rewrites::fn_00e5c9e0::rw_00e5c9e0(); 0 }
}

fn call_fn_00e5ca20() -> u32 {
    { rewrites::fn_00e5ca20::rw_00e5ca20(); 0 }
}

fn call_fn_00e5ca60() -> u32 {
    { rewrites::fn_00e5ca60::rw_00e5ca60(); 0 }
}

fn call_fn_00e5f020() -> u32 {
    rewrites::fn_00e5f020::rw_00e5f020()
}

fn call_fn_00e5f0e0() -> u32 {
    rewrites::fn_00e5f0e0::rw_00e5f0e0()
}

fn call_fn_00e5f260() -> u32 {
    rewrites::fn_00e5f260::rw_00e5f260()
}

fn call_fn_00e5f360() -> u32 {
    rewrites::fn_00e5f360::rw_00e5f360()
}

fn call_fn_00e5f3a0() -> u32 {
    rewrites::fn_00e5f3a0::rw_00e5f3a0()
}

fn call_fn_00e5f3e0() -> u32 {
    rewrites::fn_00e5f3e0::rw_00e5f3e0()
}

fn call_fn_00e5f420() -> u32 {
    rewrites::fn_00e5f420::rw_00e5f420()
}

fn call_fn_00e5f460() -> u32 {
    rewrites::fn_00e5f460::rw_00e5f460()
}

fn call_fn_00e5f530() -> u32 {
    rewrites::fn_00e5f530::rw_00e5f530()
}

fn call_fn_00e5f570() -> u32 {
    rewrites::fn_00e5f570::rw_00e5f570()
}

fn call_fn_00e5f5b0() -> u32 {
    rewrites::fn_00e5f5b0::rw_00e5f5b0()
}

fn call_fn_00e5f990() -> u32 {
    rewrites::fn_00e5f990::rw_00e5f990()
}

fn call_fn_00e5f9d0() -> u32 {
    rewrites::fn_00e5f9d0::rw_00e5f9d0()
}

fn call_fn_00e5fad0() -> u32 {
    rewrites::fn_00e5fad0::rw_00e5fad0()
}

fn call_fn_00e5fbb0() -> u32 {
    rewrites::fn_00e5fbb0::rw_00e5fbb0()
}

fn call_fn_00e5fbf0() -> u32 {
    rewrites::fn_00e5fbf0::rw_00e5fbf0()
}

/// The instances, in address order.
pub static INSTANCES: [Instance; 26] = [
    Instance {
        name: "rw_00e5c7e0",
        address: 0x00E5C7E0,
        base: 0x0110DF20,
        first: 0x0043EA80,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c7e0,
    },
    Instance {
        name: "rw_00e5c820",
        address: 0x00E5C820,
        base: 0x0110DF80,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c820,
    },
    Instance {
        name: "rw_00e5c860",
        address: 0x00E5C860,
        base: 0x0110DF50,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c860,
    },
    Instance {
        name: "rw_00e5c8e0",
        address: 0x00E5C8E0,
        base: 0x0110E004,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c8e0,
    },
    Instance {
        name: "rw_00e5c920",
        address: 0x00E5C920,
        base: 0x0110DFD4,
        first: 0x004576B0,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c920,
    },
    Instance {
        name: "rw_00e5c960",
        address: 0x00E5C960,
        base: 0x0110E034,
        first: 0x00457D90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c960,
    },
    Instance {
        name: "rw_00e5c9a0",
        address: 0x00E5C9A0,
        base: 0x0110E064,
        first: 0x00457D90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c9a0,
    },
    Instance {
        name: "rw_00e5c9e0",
        address: 0x00E5C9E0,
        base: 0x0110E0F4,
        first: 0x0043EA80,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5c9e0,
    },
    Instance {
        name: "rw_00e5ca20",
        address: 0x00E5CA20,
        base: 0x0110E0C4,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5ca20,
    },
    Instance {
        name: "rw_00e5ca60",
        address: 0x00E5CA60,
        base: 0x0110E124,
        first: 0x00459210,
        second: 0x00409610,
        returns: None,
        call: call_fn_00e5ca60,
    },
    Instance {
        name: "rw_00e5f020",
        address: 0x00E5F020,
        base: 0x0110ECE8,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f020,
    },
    Instance {
        name: "rw_00e5f0e0",
        address: 0x00E5F0E0,
        base: 0x0110EE5C,
        first: 0x00642150,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f0e0,
    },
    Instance {
        name: "rw_00e5f260",
        address: 0x00E5F260,
        base: 0x0110EECC,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f260,
    },
    Instance {
        name: "rw_00e5f360",
        address: 0x00E5F360,
        base: 0x0110EFD8,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f360,
    },
    Instance {
        name: "rw_00e5f3a0",
        address: 0x00E5F3A0,
        base: 0x0110EF78,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f3a0,
    },
    Instance {
        name: "rw_00e5f3e0",
        address: 0x00E5F3E0,
        base: 0x0110EFA8,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f3e0,
    },
    Instance {
        name: "rw_00e5f420",
        address: 0x00E5F420,
        base: 0x0110F038,
        first: 0x0065A4B0,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f420,
    },
    Instance {
        name: "rw_00e5f460",
        address: 0x00E5F460,
        base: 0x0110F008,
        first: 0x0065A500,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f460,
    },
    Instance {
        name: "rw_00e5f530",
        address: 0x00E5F530,
        base: 0x0110F08C,
        first: 0x0065B240,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f530,
    },
    Instance {
        name: "rw_00e5f570",
        address: 0x00E5F570,
        base: 0x0110F0C4,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f570,
    },
    Instance {
        name: "rw_00e5f5b0",
        address: 0x00E5F5B0,
        base: 0x0110F0F4,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f5b0,
    },
    Instance {
        name: "rw_00e5f990",
        address: 0x00E5F990,
        base: 0x0110F1B0,
        first: 0x00667160,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f990,
    },
    Instance {
        name: "rw_00e5f9d0",
        address: 0x00E5F9D0,
        base: 0x0110F220,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5f9d0,
    },
    Instance {
        name: "rw_00e5fad0",
        address: 0x00E5FAD0,
        base: 0x0110FF20,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5fad0,
    },
    Instance {
        name: "rw_00e5fbb0",
        address: 0x00E5FBB0,
        base: 0x0110FFC0,
        first: 0x0043EA90,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5fbb0,
    },
    Instance {
        name: "rw_00e5fbf0",
        address: 0x00E5FBF0,
        base: 0x0110FF80,
        first: 0x0066B240,
        second: 0x00409610,
        returns: Some(0),
        call: call_fn_00e5fbf0,
    },
];
