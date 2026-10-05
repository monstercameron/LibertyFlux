//! Lifted mainloop-timing callback registrations (lane l-lb2 draft).
//!
//! The original registers each timing callback through its own tiny
//! routine: pass the callback's code address to the shared registrar and
//! return its answer. The verified rewrites prove each instantiation in
//! the checker's 32-bit form; this crate restates the routine once as
//! portable Rust: no addresses (the callback is a descriptor token, the
//! registrar resolves it), no numbered callee slots (see
//! [`TimerRegistrar`]), no global state, no pointer-width dependence, and
//! `#![forbid(unsafe_code)]`.
//!
//! Future home: a module of `lf-main-loop` (the registrar is mainloop
//! timing's; the name pattern and the callback address range both point
//! there).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod desc;

/// One callback's dispatch entry: a name plus an index token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimerDesc {
    /// Registration name from the verified symbol, for debugging.
    pub name: &'static str,
    /// Token the registrar resolves to the callback.
    pub index: u32,
}

/// The registrar side: installs one callback by its entry.
pub trait TimerRegistrar {
    /// Registers `callback`, returning the registrar's answer.
    fn register(&mut self, callback: &TimerDesc) -> u32;
}

/// Register one timing callback, returning the registrar's answer.
pub fn register_callback(reg: &mut impl TimerRegistrar, desc: &TimerDesc) -> u32 {
    reg.register(desc)
}
