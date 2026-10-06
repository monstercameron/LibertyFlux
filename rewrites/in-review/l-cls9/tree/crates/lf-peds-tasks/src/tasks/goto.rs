//! The lifted shocking-event goto: one owning type for
//! `CTaskComplexShockingEventGoto`.
//!
//! The original keeps a goto task per curious ped: a subtask slot, a
//! behaviour kind, a position the liveness gate measures, a flag/mode
//! pair selecting the gate's path, a wait the target picker derives, a
//! stamp pair timing the periodic update, arming bytes and a speed the
//! constructor queries. This type owns those words as ordinary Rust data
//! and restates each verified 32-bit method with behaviour in it as a
//! method.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-taskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`](crate::tasks::registry).

use lf_core::Handle32;

use crate::tasks::{SubTask, TaskMgr, UninitTask, live_gate};

/// The entity a goto task walks towards (its mode slot).
///
/// Opaque: the lifted task tests it for null, never interprets it. It
/// becomes a real handle when its owner class lifts.
#[derive(Debug)]
pub struct GotoEntity;

/// The ped a goto task runs against (opaque identity).
///
/// Opaque for the same reason as [`GotoEntity`].
#[derive(Debug)]
pub struct GotoPed;

/// What the goto asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait GotoPool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Copy-constructs a goto from the member word.
    ///
    /// The original passes the member's address; the lift passes the
    /// member word and the proof pins the address.
    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        member: u32,
    ) -> Option<Handle32<GotoTask>>;
}

/// How the subtask's state check answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubVerdict {
    /// The check passed (or its gate bit was set): the update dispatches.
    Passed,
    /// The check refused: the update falls back and keeps the subtask.
    Refused,
}

/// What the goto periodic update asks of its helpers: the liveness probe,
/// the fallback builder, the subtask's state check and the dispatch helper.
pub trait GotoPoll {
    /// Probes the task for its ped (a set low byte keeps the subtask).
    fn probe(&mut self, ped: Option<Handle32<GotoPed>>) -> u32;

    /// Falls back for the ped after a kept subtask.
    fn fallback(&mut self, ped: Option<Handle32<GotoPed>>);

    /// Runs the subtask's state check: its gate bit, its check slot
    /// unless gated, marking it on a pass.
    fn check_subtask(
        &mut self,
        sub: Handle32<SubTask>,
        ped: Option<Handle32<GotoPed>>,
    ) -> SubVerdict;

    /// Dispatches the task for its ped after a passed check.
    fn dispatch(&mut self, ped: Option<Handle32<GotoPed>>);
}

/// A shocking-event goto task: its subtask, kind, position, gate pair,
/// wait, stamp pair, arming bytes and speed.
///
/// The 32-bit object holds these words past its header; the lifted form
/// owns them directly.
#[derive(Debug, Clone, Copy)]
pub struct GotoTask {
    /// The subtask slot: polled by the periodic update.
    subtask: Option<Handle32<SubTask>>,
    /// The behaviour kind: selects constructors and target paths.
    kind: u32,
    /// The goto position: measured by the liveness gate.
    pos: [f32; 3],
    /// The gate flag: selects the liveness path with the mode.
    flag: bool,
    /// The gate mode: the walked-towards entity when set.
    mode: Option<Handle32<GotoEntity>>,
    /// The derived wait: set by the target picker.
    wait: u32,
    /// The stamp the periodic update writes when arming.
    stamp: u32,
    /// The wait copy the periodic update times against.
    wait_copy: u32,
    /// The armed byte: enables the update's timer pre-gate.
    armed: bool,
    /// The restamp byte: stamps and clears itself on the next update.
    restamp: bool,
    /// The speed the constructor queries.
    speed: f32,
}

impl GotoTask {
    /// Builds a goto task from its words.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        subtask: Option<Handle32<SubTask>>,
        kind: u32,
        pos: [f32; 3],
        flag: bool,
        mode: Option<Handle32<GotoEntity>>,
        wait: u32,
        stamp: u32,
        wait_copy: u32,
        armed: bool,
        restamp: bool,
        speed: f32,
    ) -> Self {
        Self {
            subtask,
            kind,
            pos,
            flag,
            mode,
            wait,
            stamp,
            wait_copy,
            armed,
            restamp,
            speed,
        }
    }

    /// The subtask slot (`None` is the original's null).
    #[must_use]
    pub const fn subtask(self) -> Option<Handle32<SubTask>> {
        self.subtask
    }

    /// The behaviour kind.
    #[must_use]
    pub const fn kind(self) -> u32 {
        self.kind
    }

    /// The goto position.
    #[must_use]
    pub const fn pos(self) -> [f32; 3] {
        self.pos
    }

    /// The gate flag.
    #[must_use]
    pub const fn flag(self) -> bool {
        self.flag
    }

    /// The gate mode (`None` is the original's null).
    #[must_use]
    pub const fn mode(self) -> Option<Handle32<GotoEntity>> {
        self.mode
    }

    /// The derived wait.
    #[must_use]
    pub const fn wait(self) -> u32 {
        self.wait
    }

    /// The stamp.
    #[must_use]
    pub const fn stamp(self) -> u32 {
        self.stamp
    }

    /// The wait copy.
    #[must_use]
    pub const fn wait_copy(self) -> u32 {
        self.wait_copy
    }

    /// The armed byte.
    #[must_use]
    pub const fn armed(self) -> bool {
        self.armed
    }

    /// The restamp byte.
    #[must_use]
    pub const fn restamp(self) -> bool {
        self.restamp
    }

    /// The speed.
    #[must_use]
    pub const fn speed(self) -> f32 {
        self.speed
    }

    /// Clone slot: allocates a fresh object and constructs a copy.
    ///
    /// Restates the verified slot that asks the pool for a block from
    /// the shared manager and, when one answers, copy-constructs it
    /// from the embedded member, answering the construction. Answers
    /// null when the allocation fails, without constructing. The source
    /// object is never modified.
    pub fn clone_task(
        &self,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut impl GotoPool,
    ) -> Option<Handle32<GotoTask>> {
        let block = pool.alloc(manager)?;
        pool.construct(block, self.kind)
    }

    /// Periodic update slot: times the wait, then keeps or dispatches.
    ///
    /// Restates the verified slot. While armed, a set restamp byte
    /// stamps the tick and clears itself, and a lapsed wait skips the
    /// liveness gate. A live probe that fires falls back and keeps the
    /// subtask. Otherwise the subtask's state check runs: a refusal
    /// falls back and keeps the subtask, a pass dispatches and answers
    /// null. Panics without a subtask where the original faults reading
    /// its marks (every path but the probe-keeps path).
    pub fn poll(
        &mut self,
        ped: Option<Handle32<GotoPed>>,
        tick: u32,
        threshold: f32,
        poll: &mut impl GotoPoll,
    ) -> Option<Handle32<SubTask>> {
        let mut skip_gate = false;
        if self.armed {
            if self.restamp {
                self.stamp = tick;
                self.restamp = false;
            }
            if self.stamp.wrapping_add(self.wait_copy) <= tick {
                skip_gate = true;
            }
        }
        if !skip_gate && live_gate(self.flag, self.mode, &self.pos, threshold) {
            if poll.probe(ped) & 0xFF != 0 {
                poll.fallback(ped);
                return self.subtask;
            }
        }
        let Some(sub) = self.subtask else {
            panic!("goto poll without a subtask: the original faults reading its marks");
        };
        if poll.check_subtask(sub, ped) == SubVerdict::Refused {
            poll.fallback(ped);
            return self.subtask;
        }
        poll.dispatch(ped);
        None
    }
}
