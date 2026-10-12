//! The object-to-global copy instances the verified rewrites were checked against.
//!
//! Every value below is a parameter read from the verified rewrite's own text, not typed in by hand. The differential test runs the
//! rewrite at index `i` and the lifted routine on the same generated inputs.

use lf_core::field_slot_copy::FieldSlotCopy;

/// One instance: the symbol name of its rewrite and the family parameters it was extracted to.
#[derive(Clone, Copy, Debug)]
pub struct Named<T> {
    /// The rewrite's symbol name.
    pub name: &'static str,
    /// The parameters.
    pub spec: T,
}

/// The instances, in the order of the call table.
pub const INSTANCES: [Named<FieldSlotCopy>; 4] = [
    Named {
        name: "CCamScriptInstruction_SetFollowPedPitchLimitDown::vf2",
        spec: FieldSlotCopy {
            offset: 0x8,
            slot: 0x221,
        },
    },
    Named {
        name: "CCamScriptInstruction_SetFollowPedPitchLimitUp::vf2",
        spec: FieldSlotCopy {
            offset: 0x8,
            slot: 0x220,
        },
    },
    Named {
        name: "CCamScriptInstruction_SetFollowVehiclePitchLimitDown::vf2",
        spec: FieldSlotCopy {
            offset: 0x8,
            slot: 0x1,
        },
    },
    Named {
        name: "CCamScriptInstruction_SetFollowVehiclePitchLimitUp::vf2",
        spec: FieldSlotCopy {
            offset: 0x8,
            slot: 0x0,
        },
    },
];
