//! The indexed-read instances the verified rewrites were checked against.
//!
//! Generated from the verified rewrites by `extract24.py`: every value below was read from the rewrite's own
//! text, and the shape assertion in the generator holds for each entry. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::indexed_read::{EntryWidth, IndexedRead, ReadResult};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<IndexedRead>; 6] = [
    Named {
        name: "stream_flag_f0_get",
        spec: IndexedRead {
            base: 0xf0,
            stride: 0x1,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    Named {
        name: "stream_flag_fe_get",
        spec: IndexedRead {
            base: 0xfe,
            stride: 0x1,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    Named {
        name: "audio_table_float_at",
        spec: IndexedRead {
            base: 0xc4,
            stride: 0x4,
            width: EntryWidth::Word,
            result: ReadResult::Float,
        },
    },
    Named {
        name: "stream_entry_word",
        spec: IndexedRead {
            base: 0x98,
            stride: 0x10,
            width: EntryWidth::Word,
            result: ReadResult::Integer,
        },
    },
    Named {
        name: "stream_entry_flag",
        spec: IndexedRead {
            base: 0x94,
            stride: 0x10,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    Named {
        name: "stream_entry_value_float",
        spec: IndexedRead {
            base: 0x9c,
            stride: 0x10,
            width: EntryWidth::Word,
            result: ReadResult::Float,
        },
    },
];
