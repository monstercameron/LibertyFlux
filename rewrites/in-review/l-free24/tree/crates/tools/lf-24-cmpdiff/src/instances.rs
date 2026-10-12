//! The field-comparison instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The
//! differential test runs the rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::field_compare::{Compare, FieldCompare, Operand};

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<FieldCompare>; 5] = [
    Named {
        name: "camera_record_scalar_not_above_zero",
        spec: FieldCompare {
            offset: 0xed4,
            operand: Operand::Float,
            compare: Compare::Greater,
            constant: 0x0,
            negate: true,
        },
    },
    Named {
        name: "CTaskComplexGoToPointAndStandStillTimed::vf10",
        spec: FieldCompare {
            offset: 0x18,
            operand: Operand::Float,
            compare: Compare::GreaterEqual,
            constant: 0x3f800000,
            negate: false,
        },
    },
    Named {
        name: "CTaskSimpleMoveGoToPointOnRoute::vf10",
        spec: FieldCompare {
            offset: 0x18,
            operand: Operand::Float,
            compare: Compare::Greater,
            constant: 0x0,
            negate: false,
        },
    },
    Named {
        name: "scrolling_menu_value_is_positive",
        spec: FieldCompare {
            offset: 0x208,
            operand: Operand::Signed,
            compare: Compare::Greater,
            constant: 0x0,
            negate: false,
        },
    },
    Named {
        name: "scrolling_menu_value_is_negative",
        spec: FieldCompare {
            offset: 0x208,
            operand: Operand::Signed,
            compare: Compare::Less,
            constant: 0x0,
            negate: false,
        },
    },
];
