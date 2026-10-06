//! The lifted replay bar: a playback timeline scrubber.
//!
//! The original keeps one control object of about 260 bytes behind the
//! eighteen free routines named `replay_bar_*`: a time range ([`ReplayBar`]
//! holds `lo`/`hi`), a slot count (`total`), a pointer table of stamped
//! slot entries ([`ReplaySlot`]), selection state (`selected`, written to
//! two words, plus a marker word), two marker bounds, two hit rectangles,
//! a cursor word, a clock link, and the bar geometry (`origin`, `weight`,
//! `width`) with a seconds field. Sixteen of the eighteen routines are
//! methods on that object; the other two shapes under the same name
//! prefix are the notifier control ([`NotifyCtl`], whose `this` touches a
//! different word) and the clock blend ([`blend_factors`], a free function
//! with no `this` at all).
//!
//! Each type owns its data as ordinary Rust (floats, words, vectors of
//! entries; unlifted objects travel as opaque [`Handle32`] cookies) and
//! each verified 32-bit routine with behaviour in it is restated as a
//! method on it. Collaborators the rewrites reach through callee slots or
//! virtual slots (the stamp scorer, the samplers, the clock readers, the
//! selection watcher, the publisher, the notifier hub) are one small trait
//! each with blanket implementations for closures, so dispatch is static
//! and tests script answers through fakes. Shared globals travel as plain
//! parameters ([`TimeBases`], [`StampPublish`], the game-state word).
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results, every written
//! byte, and every collaborator call in order, floats bit for bit (see
//! the `lf-replaydiff` test crate). Nothing here is verified by the
//! checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].
//!
//! [`Handle32`]: lf_core::boundary::Handle32

#![forbid(unsafe_code)]

pub mod registry;

mod bar;
mod clock;
mod notify;

pub use bar::{Publish, ReplayBar, ReplaySlot, Sample, ScoreStamp, SelectionWatch, StampPublish};
pub use clock::{BaseSelect, ClockHandle, ClockRead, ClockTag, TimeBases, blend_factors};
pub use notify::{
    InnerHandle, InnerTag, NotifierHandle, NotifierTag, NotifyCtl, NotifyHub, RefreshInner,
    RefreshOutcome,
};
