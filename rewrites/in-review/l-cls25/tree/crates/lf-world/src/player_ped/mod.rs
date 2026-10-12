//! Lifted player pedestrian class: the verified methods that are restated
//! as Rust.
//!
//! The original keeps one pedestrian object per player character. Its
//! fields used here are plain words, floats and pointers; several of its
//! virtual slots call into collaborators (the sub-objects it notifies, the
//! job and model services, the context lookup) and read or write the
//! pedestrian itself while doing so. Ten of its virtual slots have verified
//! 32-bit rewrites with behaviour in them, and [`CPlayerPed`] restates
//! those ten:
//!
//! - the ready-gated job pair ([`CPlayerPed::run_jobs_if_ready`]);
//! - the linked notification ([`CPlayerPed::notify_while_linked`]);
//! - the motion value addition ([`CPlayerPed::add_motion_value`]);
//! - the reset-and-apply step ([`CPlayerPed::reset_and_apply`]);
//! - the sub-object notification pair ([`CPlayerPed::notify_sub_objects`]);
//! - the setup-and-store step ([`CPlayerPed::setup_and_store`]);
//! - the model change ([`CPlayerPed::change_model`]);
//! - the sub-object entry chain ([`CPlayerPed::run_entry_chain`]);
//! - two copies of four-word blocks ([`CPlayerPed::set_transform`] and
//!   [`CPlayerPed::set_descriptor`]).
//!
//! [`CPlayerPed`] owns the fields those methods read or write, as ordinary
//! Rust data: no addresses, no virtual tables, no globals and no pointer
//! width. Everything the methods reach outside the object (the unlifted
//! callees, the virtual dispatch to a sub-object, the global context and
//! model table) is one method each on [`PlayerPedWorld`], in the order the
//! rewrites call it. Identities the original passes as addresses are opaque
//! [`lf_core::Handle32`] cookies; a value the original passes as zero stays
//! `None` or the zero word it is.
//!
//! Each method is proven against its verified 32-bit rewrite by the
//! `lf-playerped-diff` test crate: the result, every field the rewrite may
//! change, and every collaborator call in order. Nothing here is verified by
//! the checker itself. What each method covers, and what it narrows, is
//! recorded per method in [`registry`].

#![forbid(unsafe_code)]

pub mod ped;
pub mod registry;
pub mod world;

pub use ped::{CPlayerPed, ObjRef, offsets};
pub use world::{ByteGlobal, GateWorld, ObjTag, PedTag, PlayerPedWorld, WordGlobal};
