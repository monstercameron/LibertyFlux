//! Lifted seek-cover task classes: the cover-seeking composite task and
//! its shooting variant.
//!
//! Each class owns its object's words as ordinary Rust fields. Each
//! verified method with behaviour in it is restated as a method that
//! takes a [`SeekCoverWorld`] (or the shooting world) for its callees and
//! its collaborators, so the lifted code contains no address, no numbered
//! slot and no global state. The registry lists every method, whether it
//! is proven, and what the proof narrows.
//!
//! Not every method is lifted. A method that is missing from the registry
//! as proven is missing here; nothing is written from a name alone.

#![forbid(unsafe_code)]

pub mod registry;
pub mod shooting;
pub mod task;

pub use shooting::{
    InitArg, PursuitNode, PursuitTarget, ShootingTask, ShootingWorld, ShotNodeArgs, ShotSubject,
};
pub use task::{
    CoverRowArgs, CoverRoute, CoverSubTask, CoverWorker, FullArgs, OwnedSlot, RouteArgs,
    RouteExtra, SeekCoverTask, SeekCoverWorld, ShortArgs, SmallNode, Stamp, TaskNode,
};
