//! Lifted `audio_voice` free functions: parameter blocks, voice trackers,
//! voice slots and the banked voice table.
//!
//! The verified rewrites named `audio_voice_*` belong to no class; triage
//! over all 78 (see the lane report) groups them into thirteen shapes by
//! the data their first argument points to. Four shapes are lifted here:
//!
//! - [`params`]: the five-block voice parameter bank (`(0, 1.0, 0)`
//!   defaults, the sub-object reinit) and the two-word voice header reset.
//! - [`tracker`]: the voice tracker (`+0x30` voice, `+0x34` link, `+0x38`
//!   count): install, release-and-park, install-and-bind, handle resolve.
//! - [`slots`]: the flat voice-slot file: 96-byte records behind index
//!   bytes, flag checks, slot update with notify, clear-by-id, flag advance.
//! - [`banked`]: the banked voice table behind the scale/table globals:
//!   flag set, slot store, flag-bit select.
//!
//! Each type owns its data as ordinary Rust (no addresses, no numbered
//! slots, no global state, no pointer-width dependence) and each verified
//! 32-bit routine with behaviour in it is restated as a method on it.
//! Collaborators the rewrites reach through numbered callee slots become
//! methods of one trait per structure (`SubInit`, [`TrackerWorld`],
//! [`slots::Notify`]); values owned by code not yet lifted travel as
//! opaque [`Handle32`] cookies. Proof is differential: every lifted method
//! runs against its verified rewrite on the same generated inputs,
//! comparing results, every effect and every collaborator call in order
//! (see the `lf-audiovoicediff` test crate). Nothing here is verified by
//! the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`Handle32`]: lf_core::Handle32
//! [`TrackerWorld`]: tracker::TrackerWorld

#![forbid(unsafe_code)]

pub mod banked;
pub mod params;
pub mod registry;
pub mod slots;
pub mod tracker;

pub use banked::{BankRecord, BankedVoices, VoiceSel};
pub use params::{ParamBlock, ParamBlocks, SubInit, VoiceHead};
pub use slots::{Notify, VoiceSlots};
pub use tracker::{LinkedSlot, PoolTag, TrackerWorld, VoiceHandle, VoiceTracker, VoiceTag};
