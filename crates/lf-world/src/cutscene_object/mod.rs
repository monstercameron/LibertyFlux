//! The lifted cutscene object: what a cutscene places in the world and drives.
//!
//! The original keeps one object per cutscene participant: an inline triple
//! and an optional attached placement record, a helper member, flag words,
//! a bound radius with a local corner pair, a member link, and a mode word
//! selecting how draw commands are emitted. Here those are plain fields on
//! [`CutsceneObject`], with collaborator objects carried as opaque cookies
//! and reached through [`CutsceneWorld`].
//!
//! Every method below is one verified 32-bit method restated as ordinary
//! Rust (no addresses, no virtual tables, no allocator calls): the
//! mode predicates, the flag test, the bound accessors, the triple copy,
//! the pose-record copies through the object's own slots, the word
//! forwarder, the flag-gated refresh, the helper reset, the draw-command
//! dispatch, and the two bounds computations (the world-space box and the
//! four-corner rectangle). Proof is differential: every lifted method runs
//! against its verified rewrite on the same generated inputs, comparing
//! results, every effect, and every collaborator call in order, floats bit
//! for bit (see the `lf-cutdiff` test crate). Nothing here is verified by
//! the checker itself.
//!
//! The virtual slots, as established from the verified rewrites (Verified
//! per method; the cross-method reading is Inferred):
//!
//! | Slot (byte) | Meaning |
//! |---|---|
//! | 0x00 | deleting destructor (Drop covers it) |
//! | 0x24/0x28 | the object's own guard slots read by the per-frame update |
//! | 0x54 | the object's own pose-record slot (answers a 16-byte record) |
//! | 0x58 | the object's own follow-up slot after a pose copy |
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`CutsceneWorld`]: object::CutsceneWorld

#![forbid(unsafe_code)]

pub mod object;
pub mod registry;

pub use object::{
    Accumulator, AttachTag, BlockTag, BoneRow, BoneTag, BoundsRect, BoundsScale, ChainTag, CtxTag,
    CutsceneObject, CutsceneWorld, DrawTag, EarlyTag, EntryTag, HelperTag, Matrix34, MemberTag,
    PlacementTag, PoseRecord, UpdateEntry, UpdateScalars, WorldBounds,
};
