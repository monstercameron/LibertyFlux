//! Lifted audio-entity routines: gates, event rows and object lifecycle.
//!
//! The `audio_entity_*` verified functions are free functions, not a
//! class: one name prefix over several small data shapes. Each shape
//! below owns its data as ordinary Rust (no addresses, no numbered
//! callee slots, no global state, no pointer-width dependence) and each
//! verified routine with behaviour in it is restated as a method:
//!
//! - [`EntityRecord`](record::EntityRecord): one audio entity's gate
//!   fields (id word, two mute bytes, flag dword), with the audibility
//!   and activity gates as methods. The five playback globals the gates
//!   read (mode, current id, three alternates) travel as one
//!   [`PlaybackState`](record::PlaybackState) argument.
//! - [`EventTable`](events::EventTable): the entity's table of event
//!   rows, each row holding up to twelve
//!   [`EventRecord`](events::EventRecord)s, with the append routine as
//!   a method. Row addresses narrow to row indexes.
//! - [`AudioEntity`](lifecycle::AudioEntity): the constructed object
//!   (a cleared state word plus two embedded members), built through
//!   the [`BaseInit`](lifecycle::BaseInit) and
//!   [`MemberInit`](lifecycle::MemberInit) traits, and the one-shot
//!   [`LatchSlot`](lifecycle::LatchSlot), whose latched global arrives
//!   as a plain argument.
//!
//! Proof is differential: every lifted method runs against its verified
//! 32-bit rewrite on the same generated inputs, comparing results and
//! every effect (see the `lf-audioentitydiff` test crate). Nothing here
//! is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per routine in [`registry`]. The thirteen
//! `audio_entity_init_N` routines are not lifted: each one's whole
//! content is one static-object call plus one registrar call, with no
//! behaviour in the routine itself.

#![forbid(unsafe_code)]

pub mod events;
pub mod lifecycle;
pub mod record;
pub mod registry;
