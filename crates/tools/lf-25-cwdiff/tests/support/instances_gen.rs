//! The constant-write instances of the batch, generated from the verified rewrites.
//!
//! Each entry gives a rewrite's export name and address, the writes it makes in
//! order (offset from the object, width, constant), and the return its contract
//! compares. The lifted routine is `lf_core::constant_writes::ConstantWrites`.

use lf_25_cwdiff::rewrites;
use lf_core::constant_writes::{ConstantWrite, ConstantWrites, Returns, WriteWidth};

/// One verified rewrite of the batch and the lifted routine's data for it.
#[allow(dead_code)]
pub struct NamedWrites {
    /// The export name of the rewrite.
    pub name: &'static str,
    /// The image address of the rewrite, for the report.
    pub address: u32,
    /// What the routine writes and returns.
    pub writes: ConstantWrites,
}

/// Calls each rewrite with the object's address.
fn call_fn_0056c330(this: u32) -> u32 {
    rewrites::fn_0056c330::rw_0056c330(this)
}

fn call_fn_0058c2f0(this: u32) -> u32 {
    rewrites::fn_0058c2f0::rw_0058C2F0(this)
}

fn call_fn_00648a80(this: u32) -> u32 {
    rewrites::fn_00648a80::rw_00648a80(this as *mut u8)
}

fn call_fn_00648ca0(this: u32) -> u32 {
    rewrites::fn_00648ca0::rw_00648ca0(this as *mut u8)
}

fn call_fn_00648ee0(this: u32) -> u32 {
    rewrites::fn_00648ee0::rw_00648ee0(this as *mut u8)
}

fn call_fn_0066db10(this: u32) -> u32 {
    rewrites::fn_0066db10::rw_0066db10(this)
}

/// The instances, in address order.
pub static INSTANCES: [NamedWrites; 6] = [
    NamedWrites {
        name: "rw_0056c330",
        address: 0x0056C330,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00FDECC4,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0xC,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Word,
                    value: 0x00FD6864,
                },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "rw_0058C2F0",
        address: 0x0058C2F0,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00FD29E4,
                },
                ConstantWrite {
                    offset: 0xC,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Word,
                    value: 0x00FDD154,
                },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "rw_00648a80",
        address: 0x00648A80,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Half,
                    value: 0x00000100,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00FE2B24,
                },
                ConstantWrite {
                    offset: 0x20,
                    width: WriteWidth::Byte,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x12,
                    width: WriteWidth::Byte,
                    value: 0x00000000,
                },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "rw_00648ca0",
        address: 0x00648CA0,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Half,
                    value: 0x00000100,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00FE2CD4,
                },
                ConstantWrite {
                    offset: 0x20,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x24,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x12,
                    width: WriteWidth::Byte,
                    value: 0x00000003,
                },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "rw_00648ee0",
        address: 0x00648EE0,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Half,
                    value: 0x00000100,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00FE2C44,
                },
                ConstantWrite {
                    offset: 0x20,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x24,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x12,
                    width: WriteWidth::Byte,
                    value: 0x00000006,
                },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "rw_0066db10",
        address: 0x0066DB10,
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x30,
                    width: WriteWidth::Byte,
                    value: 0x00000001,
                },
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x3C888889,
                },
                ConstantWrite {
                    offset: 0xC,
                    width: WriteWidth::Word,
                    value: 0x42700000,
                },
                ConstantWrite {
                    offset: 0x14,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x28,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4C,
                    width: WriteWidth::Word,
                    value: 0x3F800000,
                },
            ],
            returns: Returns::Zero,
        },
    },
];

/// The calls, in the same order as INSTANCES.
pub static CALLS: [fn(u32) -> u32; 6] = [
    call_fn_0056c330,
    call_fn_0058c2f0,
    call_fn_00648a80,
    call_fn_00648ca0,
    call_fn_00648ee0,
    call_fn_0066db10,
];
