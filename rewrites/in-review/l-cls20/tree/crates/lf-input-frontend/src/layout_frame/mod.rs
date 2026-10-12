//! The layout frame of the front end's interface toolkit.
//!
//! [`UILayoutFrame`] owns the bytes of the frame's state that the verified
//! methods touch and restates the ones that call no other code. [`registry`]
//! says which of the frame's verified methods are restated, which are proven
//! against their rewrites, and what each proof leaves out. The frame's
//! methods that call out (token resolution, follow-up steps, shared notifiers,
//! list release) are not here yet; they need a trait for their callees.
//!
//! No `unsafe` and no operating-system call in this module. The test crate
//! `lf-layoutframe-diff` compares each method with its verified rewrite on the
//! 32-bit target.

pub mod blend;
pub mod follow_up;
pub mod frame;
pub mod registry;

pub use frame::{SPAN, UILayoutFrame, field};
