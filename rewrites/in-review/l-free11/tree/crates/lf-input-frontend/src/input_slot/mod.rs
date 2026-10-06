//! Lifted input slots: the frontend's handle-table objects.
//!
//! The original keeps one global handle table of input slot objects (each
//! either a big record holding every field or a small record carrying only
//! a kind byte), plus a static default index that readers fall back to when
//! a slot's kind byte is zero. [`SlotStore`] owns that table: each verified
//! 32-bit routine with behaviour in it is restated as a method, and every
//! callee the routines call becomes a method on a trait ([`SlotBuild`],
//! [`NotifySinks`], [`FormatPayload`], [`AnnounceSink`], [`SlotLookup`],
//! [`SlotDrop`], [`SlotRelease`]). Small one- and two-routine structures
//! share [`singles`]: the allocation registry and the key table.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results, every written
//! byte and every collaborator call in order (see the `lf-inputslot-diff`
//! test crate). Nothing here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod singles;
mod slots;

pub mod registry;

pub use singles::{KeyTable, RegEntry, RegBuild, SlotRegistry, REG_ENTRY_SIZE};
pub use slots::{
    Announce, AnnounceSink, DestroyOutcome, FormatPayload, NotifySinks, NotifyTarget, SlotBuild,
    SlotDrop, SlotLookup, SlotObject, SlotRelease, SlotStore, ThreadEntry, ANNOUNCE_BIT,
    ANNOUNCE_FLAG_OFF, BIG_SIZE, CLEAR_OFF, DEVICE_OFF, FLAG_OFF, HI_MASK, KIND_OFF, MODE_OFF,
    NOTIFY_ARG_OFF, PAYLOAD_LEN, PAYLOAD_OFF, SMALL_SIZE, TABLE_LEN, THREAD_OWNED_OFF,
};
