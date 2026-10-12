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
pub const INSTANCES: [Named; 3] = [
    Named {
        name: "stream_set_ready",
        base: 0x150e0d1,
        spec: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Byte,
                    value: 0x1,
                },
                ConstantWrite {
                    offset: 0x3,
                    width: WriteWidth::Word,
                    value: 0x0,
                },
            ],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "reset_stream_registry_counts",
        base: 0x154e140,
        spec: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0x0,
                },
                ConstantWrite {
                    offset: 0x18,
                    width: WriteWidth::Half,
                    value: 0x0,
                },
            ],
            returns: Returns::Zero,
        },
    },
    Named {
        name: "timing_record_clear_one",
        base: 0x1a02244,
        spec: ConstantWrites {
            writes: &[
                ConstantWrite {
                    offset: 0x0,
                    width: WriteWidth::Word,
                    value: 0xffffffff,
                },
                ConstantWrite {
                    offset: 0x4,
                    width: WriteWidth::Half,
                    value: 0x0,
                },
            ],
            returns: Returns::Zero,
        },
    },
];
