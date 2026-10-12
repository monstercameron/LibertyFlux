//! Lifted base sound class: the interface every kind of sound implements.
//!
//! Lifted from the verified rewrites of `rage::audSound`: the second
//! constructor, the deleting destructor, the lookups by hash and by name,
//! the store and forwarding helpers and the link forwarding method. The
//! sound's fields are plain Rust data ([`AudSound`]); the engine around it
//! (globals, numbered callees, virtual slots, and the stored-slot writes)
//! is one trait, [`SoundWorld`], whose methods are called with the
//! original's arguments in the original's order.
//!
//! Not lifted: the default constructor and the two playback-time getters
//! (no verified rewrite yet) and the always-false slot (a constant with no
//! behaviour in it). The [`registry`] lists every method with its state and
//! what each proof narrows.
//!
//! The attach and advance methods run on voice records: the record is a
//! raw word the voice table yields, and the record's bytes are reached
//! through [`SoundWorld`] (so the proof sees every access).
//!
//! The crate root is `#![forbid(unsafe_code)]`-clean: this module holds no
//! `unsafe` of its own.

#![forbid(unsafe_code)]
//!
//! Nothing here is verified by the checker itself: the lifted methods are
//! proven against their verified rewrites by the `lf-soundbase-diff` test
//! crate, and the host tests check the edge cases on this machine.

mod audsound;
pub mod registry;
mod world;

pub use audsound::{
    AudSound, DISABLE_BIT, FAST_BIT, INIT_DONE, KIND_FAST, LANE_SELECTOR_FIELD, NEEDS_INIT,
    NO_VOICE, OPTION_FIELD, OPTION_FULL, RECORD_FLAGS, RECORD_FLAGS_MASK, ROW_BIAS_INIT,
    ROW_BIAS_LOOKUP, ROW_STRIDE, SEQ_UNSET, SLOT_COUNT, STAMP_FIELD, STEP_SCALE, TICK_OFFSET,
    VELOCITY_PAD,
};
pub use world::{FallbackTag, FrameTag, ObjectTag, RegistryTag, SlotTag, SoundWorld};
