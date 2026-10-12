//! Lifted game camera class: the deleting destructor, the restart and the
//! idle reset.
//!
//! The original keeps one camera object per active camera. Its header
//! holds the table pointer, followed by the owner, a mode word, a timer,
//! a zoom word and a block of flag bytes and pending words. Three of its
//! virtual slots have verified 32-bit rewrites with behaviour in them:
//!
//! - the deleting destructor, which runs the class's destructor body and,
//!   when the low flag bit is set, returns the object to the pool;
//! - the restart, which tears the camera down, builds a worker for the
//!   owner, chains the worker's flags, releases the worker and clears the
//!   active marker and the pending words;
//! - the idle reset, which clears the mode, timer and flag words, copies
//!   the shared default word, runs the shared reset step and then sets the
//!   idle marker and the default zoom.
//!
//! [`CCamGame`] owns the fields those three methods read or write, as
//! ordinary Rust data: no addresses, no virtual tables, no globals and no
//! pointer width. Everything the methods reach outside the object (the
//! unlifted callees, the object's own dispatched teardown, the worker's
//! release, the pool and the shared default word) is one method each on
//! [`CamGameWorld`], in the order the rewrites call it. Identities that the
//! original passes as addresses are opaque [`lf_core::Handle32`] cookies.
//!
//! Each method is proven against its verified 32-bit rewrite by the
//! `lf-camgame-diff` test crate: results, every field the method may
//! change, and every collaborator call in order. Nothing here is verified
//! by the checker itself. What each method covers, and what it narrows, is
//! recorded per method in [`registry`].

#![forbid(unsafe_code)]

pub mod game;
pub mod registry;
pub mod world;

pub use game::{ACTIVE_MARKER, CCamGame, DEFAULT_ZOOM, IDLE_MARKER, IDLE_MODE, IDLE_TIMER};
pub use world::{
    CamGameWorld, CamGlobal, CamOwnerTag, CamTag, GateBlock, GateTag, PoolTag, SingletonField,
    SingletonTag, TailTag, TargetTag, WorkerTag,
};
