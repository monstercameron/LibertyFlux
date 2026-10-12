//! The indexed-slot rewrites the lifted routine was checked against: one entry per rewrite instance.
//!
//! Generated from the verified rewrites: each offset is the parameter read from that rewrite's body.

use lf_26_assigndiff::rewrites::{m_008e53b0, m_008e5390, m_008e5440, m_008e5460};

/// One rewrite instance: its symbol name, its two field offsets, and the rewrite itself.
pub struct Instance {
    /// The routine's symbol name.
    pub name: &'static str,
    /// Byte offset of the index field.
    pub index_offset: usize,
    /// Byte offset of the value field.
    pub value_offset: usize,
    /// The verified rewrite, called with the object's address and the table index.
    pub rewrite: extern "thiscall" fn(u32, u32) -> u32,
}

/// The instances, in the order the test program runs them.
pub const INSTANCES: [Instance; 4] = [
    Instance {
        name: "audio_event_slot_assign_a",
        index_offset: 1192,
        value_offset: 1172,
        rewrite: m_008e5390::rw_008e5390,
    },
    Instance {
        name: "audio_event_slot_assign_b",
        index_offset: 1196,
        value_offset: 1176,
        rewrite: m_008e53b0::rw_008e53b0,
    },
    Instance {
        name: "audio_event_slot_assign_c",
        index_offset: 1184,
        value_offset: 1164,
        rewrite: m_008e5440::rw_008e5440,
    },
    Instance {
        name: "audio_event_slot_assign_d",
        index_offset: 1188,
        value_offset: 1168,
        rewrite: m_008e5460::rw_008e5460,
    },
];
