//! The indexed-read instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
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
pub const INSTANCES: [Named<IndexedRead>; 1] = [
    Named {
        name: "audEmitter_slot_flag_check",
        spec: IndexedRead {
            base: 0xc8,
            stride: 0xd0,
            width: EntryWidth::Word,
            result: ReadResult::Integer,
        },
    },
];
