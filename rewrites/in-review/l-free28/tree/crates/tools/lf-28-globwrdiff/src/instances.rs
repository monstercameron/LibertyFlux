//! The global constant-write instances the verified rewrites were checked against.
//!
//! Every value below is read from the verified rewrite's own text, not typed in by hand. `base` is the lowest
//! global the routine writes: the test region starts there, and each write's offset is its global's distance from it.

use lf_core::constant_writes::{ConstantWrite, ConstantWrites, Returns, WriteWidth};

/// One instance: the symbol name of its rewrite, its region base and its writes.
#[derive(Clone, Copy, Debug)]
pub struct Named {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The global address of the region's first byte.
    pub base: u32,
    /// The writes and the return.
    pub spec: ConstantWrites,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named; 5] = [
    Named {
        name: "stream_state_clear",
        base: 0x1172f53,
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x1, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x1a5, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "radar_scratch_clear",
        base: 0x118f4bd,
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x1, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0xb, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0xf, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x17, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x1b, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x13, width: WriteWidth::Word, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "reset_the_streaming_slot_state_words",
        base: 0x1036f14,
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x1719f6, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x16e0a3, width: WriteWidth::Byte, value: 0x0 }, ConstantWrite { offset: 0x1719f7, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "reset_the_streaming_mode_state_and",
        base: 0x11d6fc4,
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0x1 }, ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x8, width: WriteWidth::Byte, value: 0x0 }],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "reset_state_words",
        base: 0x104004c,
        spec: ConstantWrites {
            writes: &[ConstantWrite { offset: 0x5c1090, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x5d4c34, width: WriteWidth::Word, value: 0x0 }, ConstantWrite { offset: 0x0, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x4, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0x8, width: WriteWidth::Word, value: 0xffffffff }, ConstantWrite { offset: 0xc, width: WriteWidth::Word, value: 0xffffffff }],
            returns: Returns::Zero,
        },
    },
];
