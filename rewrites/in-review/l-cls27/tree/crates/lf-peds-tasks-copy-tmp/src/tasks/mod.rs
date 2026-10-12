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
mod flee;
mod goto;
mod hit;

pub mod registry;

pub use duck::{
    DuckEvent, DuckPed, DuckPedSide, DuckPool, DuckQuery, DuckSink, DuckSpec, DuckTask,
    DuckTaskSide, FINISH_FLAG, QUERY_CODE, QUERY_STATE, SUSTAIN_FLAG,
};
pub use fist::{DAMP_RATE_BITS, FistHeld, FistLink, FistPool, FistTarget, ShakeFist};
pub use flee::{
    EventChild, FleeAnswer, FleeEntity, FleeEvent, FleePed, FleePoll, FleePool, FleeProbe,
    FleeReact, FleeSpawn, FleeTarget, FleeTask, ReactPed, Reaction, ReactionTask,
};
pub use goto::{
    GotoBase, GotoChild, GotoEntity, GotoPed, GotoPick, GotoPoll, GotoPool, GotoTask, SubVerdict,
};
pub use hit::{HitBase, HitHandler, HitPool, HitResponse, HitStart};

use lf_core::Handle32;

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

/// A subtask owned or built by a complex task (opaque identity).
///
/// The flee and goto tasks poll the subtask in their subtask slot and
/// build fresh ones from their spawners; the lifted tasks compare and
/// carry these, never interpret them. It becomes a real handle when the
/// subtask classes lift.
#[derive(Debug)]
pub struct SubTask;

/// The shocking-event family's liveness gate.
///
/// A set flag with a set mode fires at once, mixed flag/mode never
/// fires, and a clear flag with a clear mode fires only when the squared
/// length of the position exceeds the threshold. The float order is the
/// original's, pinned against reordering so results match bit for bit.
pub(crate) fn live_gate<T: ?Sized>(
    flag: bool,
    mode: Option<Handle32<T>>,
    pos: &[f32; 3],
    threshold: f32,
) -> bool {
    if flag {
        mode.is_some()
    } else if mode.is_some() {
        false
    } else {
        let sq = add(
            add(mul(pos[0], pos[0]), mul(pos[1], pos[1])),
            mul(pos[2], pos[2]),
        );
        sq > threshold
    }
}

/// Pinned-order float multiply for the liveness gate.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// Pinned-order float add for the liveness gate.
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}
