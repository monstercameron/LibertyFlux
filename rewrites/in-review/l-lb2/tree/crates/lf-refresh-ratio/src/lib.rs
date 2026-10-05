//! Lifted stored-ratio refresh slots (lane l-lb2 draft).
//!
//! The original refreshes each stored ratio through its own tiny routine:
//! divide two global floats and store the quotient into a third global.
//! The verified rewrites prove each instantiation in the checker's 32-bit
//! form; this crate restates the routine once as portable Rust over a
//! [`RatioBank`] holding every slot: no addresses (slots are indexed by
//! the descriptor token), no global state (the bank is passed in), no
//! pointer-width dependence, and `#![forbid(unsafe_code)]`.
//!
//! The division is one single-precision operation, as in the original; NaN
//! payloads are left to the hardware per the lift method (any NaN equals
//! any NaN in the comparison).
//!
//! The owning subsystem is not established yet (the globals' owner is
//! unknown), so this family keeps its own crate until review relocates it.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod desc;

/// One ratio slot's dispatch entry: a name plus an index token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RatioDesc {
    /// Slot name from the verified symbol, for debugging.
    pub name: &'static str,
    /// Index into [`RatioBank::slots`].
    pub index: u32,
}

/// One ratio slot: the two source values and the stored quotient.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RatioSlot {
    /// Dividend source.
    pub num: f32,
    /// Divisor source.
    pub den: f32,
    /// Stored quotient.
    pub out: f32,
}

/// Every ratio slot the family refreshes.
#[derive(Clone, Debug, PartialEq)]
pub struct RatioBank {
    /// Slots in descriptor order.
    pub slots: Vec<RatioSlot>,
}

/// Refresh one stored ratio: `out = num / den`.
///
/// # Panics
///
/// When `desc.index` is outside the bank (a caller bug; every
/// differential case builds the bank in descriptor order).
pub fn refresh_ratio(bank: &mut RatioBank, desc: &RatioDesc) {
    let len = bank.slots.len();
    let slot = bank.slots.get_mut(desc.index as usize).unwrap_or_else(|| {
        panic!(
            "{}: ratio index {} outside {len} slots",
            desc.name, desc.index
        )
    });
    slot.out = slot.num / slot.den;
}
