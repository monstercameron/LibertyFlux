//! The session-hosting task: offering a hosted multiplayer session.
//!
//! Lifted from the verified 32-bit rewrites of `rage::snHostSessionTask`.
//! The task carries the gamer list it offers (two counts over one word
//! array, an extra open word, a trailer record), a countdown, a state word
//! and a probe word, plus a link to the session object it hosts. Each
//! verified method with behaviour in it is restated as a method on
//! [`HostSessionTask`]:
//!
//! - [`HostSessionTask::setup_gamer_data`]: store the list words, apply the
//!   signed count adjustment, copy the gamer array, append the trailer record.
//! - [`HostSessionTask::start`]: latch the started flag and report
//!   completion while the session is live.
//! - [`HostSessionTask::advance`]: run the countdown and the six-case state
//!   machine.
//! - [`HostSessionTask::complete`]: the completion entry behind virtual
//!   slot 7.
//!
//! The session object, the allocated host entry and the engine callees are
//! not lifted yet: the session travels as an opaque [`Handle32`] cookie and
//! every callee slot, every session-field access and every virtual call of
//! the verified rewrites is one method on [`HostSessionWorld`], so dispatch
//! is static and tests script answers through a fake. Globals the methods
//! read (the image base the descriptor tags relocate against, the cookie
//! word, the setup flag byte and the setup key) travel in
//! [`HostSessionGlobals`], filled when state is loaded.
//!
//! The virtual interface, as established from the verified rewrites
//! (slot numbers from the 32-bit method names; the cross-slot reading is
//! that slot 7 is the completion entry both other virtuals report through):
//!
//! | Slot | Meaning |
//! |---|---|
//! | 2 | start: latch once, report while live |
//! | 3 | advance: countdown plus state machine |
//! | 7 | complete: select the main, side or tail path |
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results, every effect,
//! and every collaborator call in order (see the `lf-hostsessdiff` test
//! crate). Nothing here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`Handle32`]: lf_core::Handle32

#![forbid(unsafe_code)]

pub mod registry;
pub mod task;

#[cfg(test)]
mod tests;

pub use task::{
    DESCRIPTOR_TAG_FIRST, DESCRIPTOR_TAG_SECOND, GAMER_COUNT_THRESHOLD, GAMER_WORDS,
    GamerListParams, HostSessionGlobals, HostSessionTask, HostSessionWorld, KEY_TABLE_BASE,
};

/// Tag for the opaque cookie of the hosted session object.
pub struct SessionTag;
/// Tag for the opaque cookie of an allocated host-session entry.
pub struct EntryTag;
