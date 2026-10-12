//! The indexed reads the verified rewrites were checked against: one entry per rewrite instance.
//!
//! Each entry gives the symbol name of a routine and its table: base offset, entry stride and width. The
//! test crate `lf-indexdiff` runs every entry against its verified rewrite, in this order.

use super::{EntryWidth, IndexedRead, ReadResult};

/// One routine's indexed read, with its symbol name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamedRead {
    /// The routine's symbol name.
    pub name: &'static str,
    /// The read the routine performs.
    pub read: IndexedRead,
}

/// The instances, in the order the test crate calls their rewrites.
pub const READS: [NamedRead; 6] = [
    NamedRead {
        name: "indexed_byte_fetch",
        read: IndexedRead {
            base: 0x543,
            stride: 1,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    NamedRead {
        name: "CPedModelInfo::vf14",
        read: IndexedRead {
            base: 0x134,
            stride: 1,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    NamedRead {
        name: "stream_entry_tail_float",
        read: IndexedRead {
            base: 0xA0,
            stride: 16,
            width: EntryWidth::Word,
            result: ReadResult::Float,
        },
    },
    NamedRead {
        name: "stream_sample_flag",
        read: IndexedRead {
            base: 0xC4,
            stride: 12,
            width: EntryWidth::Byte,
            result: ReadResult::Integer,
        },
    },
    NamedRead {
        name: "stream_sample_float",
        read: IndexedRead {
            base: 0xCC,
            stride: 12,
            width: EntryWidth::Word,
            result: ReadResult::Float,
        },
    },
    NamedRead {
        name: "stream_sample_word",
        read: IndexedRead {
            base: 0xC8,
            stride: 12,
            width: EntryWidth::Word,
            result: ReadResult::Integer,
        },
    },
];
