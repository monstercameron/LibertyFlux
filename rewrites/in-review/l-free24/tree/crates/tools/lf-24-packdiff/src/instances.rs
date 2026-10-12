//! The packed low-byte flag instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::packed_flag::{Condition, PackedFlag, Source};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<PackedFlag>; 5] = [
    Named {
        name: "physics_is_supported_joint_mode",
        spec: PackedFlag {
            source: Source::Field(0x14),
            base: 0x0,
            condition: Condition::OneOf(0x1, 0x3),
        },
    },
    Named {
        name: "CTaskComplexMoveGoToShelterAndWait::vf6",
        spec: PackedFlag {
            source: Source::Field(0x18),
            base: 0x0,
            condition: Condition::OneOf(0x1, 0x2),
        },
    },
    Named {
        name: "exclusive_flag_pair_check",
        spec: PackedFlag {
            source: Source::Field(0xb4),
            base: 0x0,
            condition: Condition::SetAndClear { set: 0x10, clear: 0x20 },
        },
    },
    Named {
        name: "climb_task_range_check",
        spec: PackedFlag {
            source: Source::Parameter,
            base: 0x73,
            condition: Condition::AtMost(0x7),
        },
    },
    Named {
        name: "range_check_sub3",
        spec: PackedFlag {
            source: Source::Parameter,
            base: 0x3,
            condition: Condition::AtMost(0xb),
        },
    },
];
