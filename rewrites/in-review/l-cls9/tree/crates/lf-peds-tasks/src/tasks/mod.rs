//! Lifted pedestrian tasks: small task classes that pick their next subtask.
//!
//! The original drives each pedestrian through task objects: a complex
//! task owns a subtask it polls and replaces, a simple task runs its own
//! update. This module owns six such classes' data as ordinary Rust (no
//! addresses, no virtual tables, no global state) and restates each
//! verified 32-bit method with behaviour in it as a method on its type.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-taskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

#![forbid(unsafe_code)]

mod duck;
mod fist;
mod hit;

pub mod registry;

pub use duck::{
    DuckEvent, DuckPed, DuckPedSide, DuckPool, DuckQuery, DuckSink, DuckSpec, DuckTask,
    DuckTaskSide,
};
pub use fist::{DAMP_RATE_BITS, FistHeld, FistLink, FistPool, FistTarget, ShakeFist};
pub use hit::{HitBase, HitHandler, HitPool, HitResponse, HitStart};

/// The shared task-pool manager word the clone slots read (opaque identity).
///
/// Opaque: the lifted tasks carry it into pool calls, never interpret it.
/// It becomes a real handle when the pool manager lifts.
#[derive(Debug)]
pub struct TaskMgr;

/// A freshly allocated but not yet constructed task block (opaque identity).
///
/// The pool's allocator answers one of these; the class copy constructor
/// builds it into a task. Opaque for the same reason as [`TaskMgr`].
#[derive(Debug)]
pub struct UninitTask;
