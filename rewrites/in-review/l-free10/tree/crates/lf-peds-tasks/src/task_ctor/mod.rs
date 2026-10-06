//! The task constructors: every verified `task_ctor_*` free function, lifted.
//!
//! The list held 32 verified free functions in five clusters (see the
//! triage table in the lane report). They share no class, but each
//! cluster shares one data shape:
//!
//! - [`TaskParams`]: the task parameter block, a plain 64-byte record
//!   the eighteen kind initialisers fill (kinds `0x2f`..`0x46`). Seven
//!   are lifted here; the rest are [`registry`](self::registry) rows.
//! - The small vtable'd task objects (three routines), the guarded
//!   follow-up builders (three), the RNG-derived task (one) and the
//!   point-carrying task objects (seven) are not lifted in this lane;
//!   their rows say what each routine does and why it waits.
//!
//! Every lifted initialiser is proven against its verified 32-bit
//! rewrite by the `lf-taskctordiff` test crate: same inputs, same
//! starting bytes, returns, every written byte and every collaborator
//! call compared, with a deliberately wrong lift caught alongside.

#![forbid(unsafe_code)]

pub mod params;
pub mod registry;

pub use params::{
    TaskInit, TaskParams, PARAM_LEN,
};
