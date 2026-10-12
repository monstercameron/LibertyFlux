//! The conditional float-store rewrites the lifted routine was checked against: one entry per rewrite instance.
//!
//! Generated from the verified rewrites: each offset is the float field read from that rewrite's body.

use lf_26_floatdiff::rewrites::{m_00cbc0b0, m_00cbc0d0, m_00cbc070, m_00cbc090};

/// One rewrite instance: its symbol name, the offset of its float field, and the rewrite.
pub struct Instance {
    /// The routine's symbol name.
    pub name: &'static str,
    /// Byte offset of the float field.
    pub offset: usize,
    /// The verified rewrite, called with the object's address and the incoming float.
    pub rewrite: extern "thiscall" fn(u32, f32),
}

/// The instances, in the order the test program runs them.
pub const INSTANCES: [Instance; 4] = [
    Instance {
        name: "CTaskComplexMoveGetToPointContinuous::vf10",
        offset: 16,
        rewrite: m_00cbc070::rw_00cbc070,
    },
    Instance {
        name: "CTaskComplexGoToPointAndStandStillTimed::vf10",
        offset: 28,
        rewrite: m_00cbc090::rw_00cbc090,
    },
    Instance {
        name: "CTaskComplexMoveGoToPointRelativeToEntityAndStandStill::vf10",
        offset: 48,
        rewrite: m_00cbc0b0::rw_00cbc0b0,
    },
    Instance {
        name: "CTaskSimpleMoveGoToPointOnRoute::vf10",
        offset: 140,
        rewrite: m_00cbc0d0::rw_00cbc0d0,
    },
];
