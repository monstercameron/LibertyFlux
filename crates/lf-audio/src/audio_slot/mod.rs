//! The audio slot file: free routines over the game's voice-slot data.
//!
//! The 57 verified `audio_slot_*` routines are free functions (no class).
//! Triage over their rewrites groups the liftable ones by the shape of the
//! data they share: banked voice slots selected by bank and slot bytes
//! ([`banked`]), the 800-slot voice list behind a bitset ([`voicelist`]),
//! small pools ([`pools`]) and single-routine neighbours ([`misc`]). The
//! [`registry`] says per routine what is proven and what each proof
//! narrows. Thirty-five registration thunks and one virtual forward hold
//! no behaviour and are not lifted; four large routines are deferred.

pub mod banked;
pub mod misc;
pub mod pools;
pub mod registry;
pub mod voicelist;
