//! The complex melee task: the ped task that runs a hand-to-hand fight.
//!
//! The family is the single class `CTaskComplexMelee` with its verified
//! 32-bit rewrites as the proof's reference; this module restates the
//! verified methods as methods of [`task::MeleeTask`]. Every call a verified
//! method makes that is not itself lifted is one method of
//! [`host::MeleeHost`]: the host performs the call and gives back its
//! answer, and the lifted method makes the same calls, with the same
//! arguments, in the same order.
//!
//! What is lifted and proven, what is not, and what each proof narrows is
//! listed in [`registry`].
//!
//! No `unsafe` code, no addresses, no numbered virtual slots and no global
//! state: the lifted type owns its data and the host supplies the rest.

#![forbid(unsafe_code)]

pub mod host;
pub mod registry;
pub mod task;

pub use host::{CtorArgs, InitWords, MeleeHost, StampWords, StartArgs};
pub use task::{
    DISABLE_BIT, FLAG_BASE, FOLLOW_UP_ALT, FREE_BIT, KIND_MELEE, MeleeTask, ResetSlot, SLOT_OFFSETS,
    SubTask,
};
