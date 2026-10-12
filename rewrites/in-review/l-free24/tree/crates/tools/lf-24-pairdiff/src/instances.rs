//! The field-pair store instances the verified rewrites were checked against.
//!
//! Generated from the verified rewrites by `extract24.py`: every value below was read from the rewrite's own
//! text, and the shape assertion in the generator holds for each entry. The differential test runs the
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
pub const INSTANCES: [Named<FieldPairStore>; 5] = [
    Named {
        name: "rm_ptfx_manager_set_words_4b0_4b4",
        spec: FieldPairStore {
            first_offset: 0x4b0,
            second_offset: 0x4b4,
            result: PairResult::Second,
        },
    },
    Named {
        name: "audio_set_fields_78_90",
        spec: FieldPairStore {
            first_offset: 0x78,
            second_offset: 0x90,
            result: PairResult::Second,
        },
    },
    Named {
        name: "audio_pair_store_17e4",
        spec: FieldPairStore {
            first_offset: 0x17e4,
            second_offset: 0x17e8,
            result: PairResult::Zero,
        },
    },
    Named {
        name: "ped_set_dwords130_134",
        spec: FieldPairStore {
            first_offset: 0x130,
            second_offset: 0x134,
            result: PairResult::Second,
        },
    },
    Named {
        name: "stream_state_set_pair",
        spec: FieldPairStore {
            first_offset: 0x110,
            second_offset: 0x114,
            result: PairResult::Second,
        },
    },
];
