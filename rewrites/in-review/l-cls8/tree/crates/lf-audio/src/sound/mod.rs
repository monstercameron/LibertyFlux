//! Lifted audio effects: what an effect does each update.
//!
//! A sound plays through a chain of effects; each effect owns a small
//! bank of parameter rows that rotates every tick. All effects share the
//! base [`Effect`] layout (info block, attached voice, index words, a
//! fifteen-slot gain table, a loop-bound byte and an enable flag); each
//! subclass adds its own parameter rows and row width:
//!
//! - [`Effect`] (`rage::audEffect`): the base: fifteen gain slots in
//!   three rows of five, a row-rotation step, attach and poll entries.
//! - [`CompressorEffect`](compressor::CompressorEffect): three
//!   nine-word parameter rows, rotated past a listener.
//!
//! Each type owns its words as ordinary Rust data (no addresses, no
//! virtual tables, no allocator calls) and each verified 32-bit method
//! with behaviour in it is restated as a method. Collaborator objects
//! (info blocks, voices, listeners, sub-objects) are carried as opaque
//! [`Handle32`] cookies and reached through one trait per class
//! (`EffectWorld`, `CompressorWorld`): every callee slot and every
//! virtual call of the verified rewrites is one trait method, so
//! dispatch is static and tests script answers through a fake.
//!
//! The effect virtual slots, as established from the verified rewrites
//! (Verified per class; the cross-class reading is Inferred):
//!
//! | Slot | Meaning |
//! |---|---|
//! | 0 | deleting destructor through the audio heap (Drop covers it) |
//! | 1 | attach or initialise (info block, gains, fan-out) |
//! | 2 | poll: refresh an entry, run the sub-object or voice |
//! | 4 | address of the current parameter row (a reference in the lift) |
//! | 5 | advance one tick: rotate a row, refresh, hand to the next stage |
//!
//! Slots are numbered by vtable byte offset divided by four. Proof is
//! differential: every lifted method runs against its verified rewrite
//! on the same generated inputs, comparing results, every effect, and
//! every collaborator call in order (see the `lf-sounddiff` test
//! crate). Nothing here is verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`Handle32`]: lf_core::Handle32

#![forbid(unsafe_code)]

pub mod compressor;
pub mod effect;
pub mod registry;

pub use compressor::{CompressorEffect, CompressorWorld};
pub use effect::{Effect, EffectWorld};

/// Tag for the opaque cookie of an effect info or preset block.
pub struct InfoTag;
/// Tag for the opaque cookie of a voice attached to an effect.
pub struct VoiceTag;
/// Tag for the opaque cookie of a compressor listener object.
pub struct ListenerTag;
/// Tag for the opaque cookie of a compressor sub-object.
pub struct SubTag;
