//! The field-pair store instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::field_pair_store::{FieldPairStore, PairResult};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<FieldPairStore>; 3] = [
    Named {
        name: "stream_pair_set",
        spec: FieldPairStore {
            first_offset: 0x1c,
            second_offset: 0x8,
            result: PairResult::Zero,
        },
    },
    Named {
        name: "audStaticRadioEmitter::vf11",
        spec: FieldPairStore {
            first_offset: 0x38,
            second_offset: 0x3c,
            result: PairResult::Zero,
        },
    },
    Named {
        name: "store_two_floats_c0",
        spec: FieldPairStore {
            first_offset: 0xc0,
            second_offset: 0xc4,
            result: PairResult::Zero,
        },
    },
];
