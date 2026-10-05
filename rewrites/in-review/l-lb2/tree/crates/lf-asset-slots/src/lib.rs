//! Lifted per-asset slot registrations (lane l-lb2 draft).
//!
//! The original registers each asset's slot through its own tiny routine:
//! call the shared slot-registration routine with the slot record and the
//! asset name, which links the slot into the registry list and returns the
//! slot pointer. The verified rewrites prove each instantiation in the
//! checker's 32-bit form; this crate restates the routine once as portable
//! Rust: no addresses (slot and name are resolved by the registry from the
//! descriptor token), no numbered callee slots (see [`AssetRegistry`]), no
//! global state, no pointer-width dependence, and `#![forbid(unsafe_code)]`.
//!
//! Future home: probably a module of `lf-streaming` (the routine links
//! slots into a registry list, which is asset bookkeeping), but the owning
//! subsystem is not established yet; relocation is expected on review.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod desc;

/// One asset's dispatch entry: a name plus an index token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetDesc {
    /// Asset name from the verified symbol, for debugging.
    pub name: &'static str,
    /// Token the registry resolves to the slot record and name.
    pub index: u32,
}

/// Opaque handle to a registered slot (the routine's answer).
///
/// Stand-in for `lf_core::Handle32` until this crate joins the workspace:
/// a 32-bit cookie the lifted code carries but never interprets.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Slot(u32);

impl Slot {
    /// Wraps a raw cookie.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// The raw cookie, for the boundary only.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// The registry side: links one asset slot by its entry.
pub trait AssetRegistry {
    /// Registers `asset`, returning its slot handle.
    fn register_slot(&mut self, asset: &AssetDesc) -> Slot;
}

/// Register one asset slot, returning its handle.
pub fn register_slot(reg: &mut impl AssetRegistry, desc: &AssetDesc) -> Slot {
    reg.register_slot(desc)
}
