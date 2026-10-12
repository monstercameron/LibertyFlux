//! The field-store instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::field_store::{FieldStore, FieldWidth, StoreResult};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<FieldStore>; 10] = [
    Named {
        name: "rage::spdApical::vf2",
        spec: FieldStore {
            offset: 0x10,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "rage::spdApical::vf3",
        spec: FieldStore {
            offset: 0x14,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "rage::spdApical::vf4",
        spec: FieldStore {
            offset: 0x18,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "rage::spdApical::vf5",
        spec: FieldStore {
            offset: 0x1c,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "rage::crmtNodePair::vf14",
        spec: FieldStore {
            offset: 0x20,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "crmt_extrapolate_node_store_rate",
        spec: FieldStore {
            offset: 0x30,
            width: FieldWidth::Word,
            result: StoreResult::Nothing,
        },
    },
    Named {
        name: "UILayoutFrame::vf49",
        spec: FieldStore {
            offset: 0x8,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    Named {
        name: "UILayoutFrame::vf47",
        spec: FieldStore {
            offset: 0x4,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    Named {
        name: "anim_set_blend_limit",
        spec: FieldStore {
            offset: 0x58,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
    Named {
        name: "anim_set_current",
        spec: FieldStore {
            offset: 0x2c,
            width: FieldWidth::Word,
            result: StoreResult::Zero,
        },
    },
];
