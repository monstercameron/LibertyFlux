//! Lifted free functions of the ped task system: pose volumes and task creation.
//!
//! The original drives each pedestrian through task objects. Most of the
//! task code belongs to no named class: these are the free functions the
//! naming lanes grouped by behaviour under `ped_task_*`. This module owns
//! the data of the three simplest shapes found among them as ordinary
//! Rust (no addresses, no virtual tables, no global state, no dependence
//! on pointer width) and restates each verified 32-bit routine with
//! behaviour in it as a method on its owning type.
//!
//! - [`pose`]: the task pose volume (two local-space vectors, a tag word,
//!   an optional parent link and a flag byte) with its world-space
//!   transform, its pose blend and its point-in-volume test.
//! - [`create`]: task creation. The kind-`0x11` task builder shared by the
//!   four-word and five-word build routines (one generic method, proven
//!   once per arity), and the entry-chain cloner shared by the two clone
//!   routines (one generic method over the done-flag write, proven once
//!   per instance).
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-pedtaskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per routine in [`registry`].

#![forbid(unsafe_code)]

mod create;
mod pose;

pub mod registry;

pub use create::{
    BuildManagers, ChainCloner, ChainClone, ChainEntry, ChainNode, ChainOwner, CloneProduct,
    FoundEntry, Kind11Task, PedMgr, PedTaskOutcome, TaskBuildCtx, FLAG_DONE, FLAG_EXTRA, KIND_11,
    NONE, PRIORITY,
};
pub use pose::{
    AngleTuning, Blended, ConeSolvers, Link, LinkMatrix, Matrix, NormSlot, PoseFill, PoseSample,
    PoseVolume, Transformed, FLAG_ACTIVE, FLAG_COPY, FLAG_SPHERE, FLAG_TRANSFORM, NORM_COUNT,
};
