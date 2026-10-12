//! The constant-write routines the verified rewrites were checked against: one entry per rewrite instance.
//!
//! Each entry gives the symbol name of a routine and its writes and return. The test crate `lf-constdiff`
//! runs every entry against its verified rewrite, in this order, and the test fails on any difference.

use super::{ConstantWrite, ConstantWrites, Returns, WriteWidth};

/// One routine's constant writes, with its symbol name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedWrites {
    /// The routine's symbol name.
    pub name: &'static str,
    /// What the routine writes and returns.
    pub writes: ConstantWrites,
}

/// The instances, in the order the test crate calls their rewrites.
pub const INSTANCES: [NamedWrites; 8] = [
    NamedWrites {
        name: "reset_state_b",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x4,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x86,
                width: WriteWidth::Half,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x0,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ],
            returns: Returns::Zero,
        },
    },
    NamedWrites {
        name: "ui_element_init_core",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x0,
                width: WriteWidth::Byte,
                value: 0x00000001,
            },
            ConstantWrite {
                offset: 0xC,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x10,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ],
            returns: Returns::Zero,
        },
    },
    NamedWrites {
        name: "Input_ResetDevice",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x4,
                width: WriteWidth::Word,
                value: 0x00000001,
            },
            ConstantWrite {
                offset: 0x0,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x8,
                width: WriteWidth::Half,
                value: 0x00000000,
            },
            ],
            returns: Returns::Zero,
        },
    },
    NamedWrites {
        name: "HandleRec_Init",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x0,
                width: WriteWidth::Word,
                value: 0xFFFFFFFF,
            },
            ConstantWrite {
                offset: 0x4,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x40,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "tag6_clear",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x0,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x1,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ConstantWrite {
                offset: 0x5,
                width: WriteWidth::Byte,
                value: 0x00000000,
            },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "ccam_sector_init",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x140,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ],
            returns: Returns::One,
        },
    },
    NamedWrites {
        name: "clear_field_0ee0",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0xEE0,
                width: WriteWidth::Word,
                value: 0x00000000,
            },
            ],
            returns: Returns::This,
        },
    },
    NamedWrites {
        name: "targeting_float_reset",
        writes: ConstantWrites {
            writes: &[
            ConstantWrite {
                offset: 0x224,
                width: WriteWidth::Word,
                value: 0x40800000,
            },
            ],
            returns: Returns::Zero,
        },
    },
];
