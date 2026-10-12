//! The three-byte store rewrites the lifted routine was checked against: one entry per rewrite instance.
//!
//! Generated from the verified rewrites. The two forms differ only in the type of the object argument
//! (an address, or a byte pointer); both are called with the object's address.

use crate::rewrites::{m_00bebdb0, m_00bebdd0, m_00bf4110, m_00bf4130};

/// The two argument forms of the verified rewrites.
pub enum Form {
    /// The object is passed as an address.
    Address(extern "thiscall" fn(u32, u32) -> u32),
    /// The object is passed as a byte pointer.
    Pointer(extern "thiscall" fn(*mut u8, u32) -> u32),
}

/// One rewrite instance: its symbol name, the offset of its lowest byte, and the rewrite.
pub struct Instance {
    /// The routine's symbol name.
    pub name: &'static str,
    /// Byte offset of the least significant of the three bytes.
    pub offset: usize,
    /// The verified rewrite in its argument form.
    pub form: Form,
}

/// The instances, in the order the test program runs them.
pub const INSTANCES: [Instance; 4] = [
    Instance {
        name: "split_u24_be_567",
        offset: 5,
        form: Form::Address(m_00bebdb0::rw_00bebdb0),
    },
    Instance {
        name: "split_u24_be_123",
        offset: 1,
        form: Form::Address(m_00bebdd0::rw_00bebdd0),
    },
    Instance {
        name: "pack_rgb_hi",
        offset: 21,
        form: Form::Pointer(m_00bf4110::rw_bf4110),
    },
    Instance {
        name: "pack_rgb_lo",
        offset: 17,
        form: Form::Pointer(m_00bf4130::rw_bf4130),
    },
];
