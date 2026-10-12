//! The block-copy rewrite the existing lifted routine was checked against.
//!
//! One entry: a six-word run copied from the caller's buffer into the object at a fixed offset, returning the
//! buffer's address. Generated from the verified rewrite's body: the offset, the word count and the direction are
//! read from it.

use lf_core::word_block::{CopyDirection, CopyReturn, WordBlockCopy};

use lf_26_blockcopydiff::rewrites::m_00c0f890;

/// One rewrite instance: its symbol name, the routine's parameters, and the rewrite itself.
pub struct Instance {
    /// The routine's symbol name.
    pub name: &'static str,
    /// The copy the routine makes.
    pub copy: WordBlockCopy,
    /// The verified rewrite, called with the object's address and the buffer's address.
    pub rewrite: extern "thiscall" fn(*mut u8, *const u8) -> u32,
}

/// The instances, in the order the test program runs them.
pub const INSTANCES: [Instance; 1] = [
    Instance {
        name: "UILayoutFrame::copy24_to_0x10",
        copy: WordBlockCopy::new(CopyDirection::IntoObject, 0x10, 6, CopyReturn::Buffer),
        rewrite: m_00c0f890::rw_00c0f890,
    },
];
