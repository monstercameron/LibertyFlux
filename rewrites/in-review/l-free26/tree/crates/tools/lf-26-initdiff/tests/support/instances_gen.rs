//! The constant-write rewrites the existing lifted routine was checked against: one entry per rewrite.
//!
//! Generated from the matcher's output: each write's offset, width and constant, and the return, are read
//! from the rewrite's own body. The writes are in the order the rewrite makes them.

use lf_core::constant_writes::{ConstantWrite, ConstantWrites, Returns, WriteWidth};

use lf_26_initdiff::rewrites::{m_00937b60, m_00a09f20, m_00c886c0};

/// One rewrite instance: its symbol name, its writes and return, and the rewrite itself.
pub struct Instance {
    /// The routine's symbol name.
    pub name: &'static str,
    /// What the routine writes and returns.
    pub writes: ConstantWrites,
    /// The verified rewrite, called with the object's address.
    pub rewrite: extern "thiscall" fn(u32) -> u32,
}

/// The instances, in the order the test program runs them.
pub const INSTANCES: [Instance; 3] = [
    Instance {
        name: "stream_unknown",
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x104,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x108,
                    width: WriteWidth::Half,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Byte,
                    value: 0x00000000,
                },
            ],
            returns: Returns::This,
        },
        rewrite: m_00937b60::rw_00937b60,
    },
    Instance {
        name: "mission_cleanup_entry_init",
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0xffffffff,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Byte,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0xc,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x14,
                    width: WriteWidth::Byte,
                    value: 0x00000000,
                },
            ],
            returns: Returns::Zero,
        },
        rewrite: m_00a09f20::rw_00a09f20,
    },
    Instance {
        name: "audio_tracker_init",
        writes: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x8,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x20,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0xc,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x10,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x14,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x18,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x1c,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Word,
                    value: 0x00000000,
                },
            ],
            returns: Returns::This,
        },
        rewrite: m_00c886c0::rw_00c886c0,
    },
];
