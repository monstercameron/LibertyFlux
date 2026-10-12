//! Lifted mirror-reflection render phase.
//!
//! The phase draws the scene reflected in a mirror. Its object keeps a
//! target handle, a parameter block handle, three filter seeds and a status
//! byte ([`MirrorReflectionPhase`]); its process-wide values live in
//! [`MirrorGlobals`], which the caller owns and lends to the methods; and
//! every collaborator it calls is one method of [`MirrorHost`].
//!
//! Each verified method with behaviour in it is restated as a method here:
//! the deleting destructor, the refresh, the reflection and the render.
//! Proof is differential: every method runs against its verified 32-bit
//! rewrite on the same generated inputs, comparing results, every call in
//! order with its arguments, and every written byte, floats bit for bit (see
//! the `lf-mirrordiff` test crate). Nothing here is verified by the checker
//! itself.
//!
//! What each method covers, and what it narrows away from the original, is
//! recorded per method in [`registry`].
//!
//! No address appears in this module, no numbered callee and no
//! `unsafe`: the object and the parameter block are reached through their
//! handles, and the parameter table and the globals are values the caller
//! supplies. Loading those values from the executable's image is not in
//! this module yet.

#![forbid(unsafe_code)]

pub mod globals;
pub mod host;
pub mod phase;
pub mod registry;

pub use globals::MirrorGlobals;
pub use host::{MirrorHost, Ref, Word};
pub use phase::MirrorReflectionPhase;
