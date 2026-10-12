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
//! (no verified rewrite yet), the always-false slot, and the attach and
//! advance slots, which are verified but still wait for their lift. The
//! [`registry`] lists every verified method with its state and narrowings.
//!
//! Nothing here is verified by the checker itself: the lifted methods are
//! proven against their verified rewrites by the `lf-soundbase-diff` test
//! crate, and the host tests check the edge cases on this machine.

mod audsound;
pub mod registry;
mod world;

pub use audsound::{
    AudSound, DISABLE_BIT, FAST_BIT, INIT_DONE, KIND_FAST, NEEDS_INIT, NO_VOICE, ROW_BIAS_INIT,
    ROW_BIAS_LOOKUP, ROW_STRIDE, SEQ_UNSET, SLOT_COUNT,
};
pub use world::{FallbackTag, ObjectTag, RegistryTag, SlotTag, SoundWorld};
