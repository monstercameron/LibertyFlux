//! The lifted shocking-event flee: one owning type for
//! `CTaskComplexShockingEventFlee`.
//!
//! The original keeps a flee task per frightened ped: a subtask slot the
//! periodic update polls and keeps, a marks word, a behaviour kind, a
//! position the liveness gate measures, a flag/mode pair selecting the
//! gate's path, and a state byte. This type owns those words as ordinary
//! Rust data and restates each verified 32-bit method with behaviour in
//! it as a method.
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

/// The entity a flee task flees from (its mode slot).
///
/// Opaque: the lifted task tests it for null and hands it to the entity
/// constructor, never interprets it. It becomes a real handle when its
/// owner class lifts.
#[derive(Debug)]
pub struct FleeEntity;

/// The ped a flee task runs against (opaque identity).
///
/// Opaque for the same reason as [`FleeEntity`].
#[derive(Debug)]
pub struct FleePed;

/// What the flee spawner asks of the task pool: the allocator and the
/// two constructor forms behind the spawn slot.
pub trait FleeSpawn {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Builds the entity form from the fleeing entity and the kind word.
    fn construct_entity(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
        kind: u32,
    ) -> Option<Handle32<SubTask>>;

    /// Builds the vector form from the flee position and the kind word.
    fn construct_vector(
        &mut self,
        block: Handle32<UninitTask>,
        pos: [f32; 3],
        kind: u32,
    ) -> Option<Handle32<SubTask>>;
}

/// What the flee periodic update asks of its helpers: the liveness probe,
/// the task's own state check and the dispatch helper.
pub trait FleePoll {
    /// Probes the task for its ped (a set low byte keeps the subtask).
    fn probe(&mut self, ped: Option<Handle32<FleePed>>) -> u32;

    /// Runs the task's own state check (a clear low byte keeps the subtask).
    fn check(&mut self, ped: Option<Handle32<FleePed>>) -> u32;

    /// Dispatches the task for its ped after a passed check.
    fn dispatch(&mut self, ped: Option<Handle32<FleePed>>);
}

/// A shocking-event flee task: its subtask, marks, kind, position, gate
/// pair and state byte.
///
/// The 32-bit object holds these words past its header; the lifted form
/// owns them directly.
#[derive(Debug, Clone, Copy)]
pub struct FleeTask {
    /// The subtask slot: polled and kept by the periodic update.
    subtask: Option<Handle32<SubTask>>,
    /// The marks word: bit 0 gates the state check, bit 1 is set when
    /// the check passes.
    marks: u32,
    /// The behaviour kind: selects constructors and reaction paths.
    kind: u32,
    /// The flee position: measured by the liveness gate.
    pos: [f32; 3],
    /// The gate flag: selects the liveness path with the mode.
    flag: bool,
    /// The gate mode: the fleeing entity when set.
    mode: Option<Handle32<FleeEntity>>,
    /// The state byte: carried into the reaction builders.
    state: u8,
}

impl FleeTask {
    /// Builds a flee task from its words.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        subtask: Option<Handle32<SubTask>>,
        marks: u32,
        kind: u32,
        pos: [f32; 3],
        flag: bool,
        mode: Option<Handle32<FleeEntity>>,
        state: u8,
    ) -> Self {
        Self {
            subtask,
            marks,
            kind,
            pos,
            flag,
            mode,
            state,
        }
    }

    /// The subtask slot (`None` is the original's null).
    #[must_use]
    pub const fn subtask(self) -> Option<Handle32<SubTask>> {
        self.subtask
    }

    /// The marks word.
    #[must_use]
    pub const fn marks(self) -> u32 {
        self.marks
    }

    /// The behaviour kind.
    #[must_use]
    pub const fn kind(self) -> u32 {
        self.kind
    }

    /// The flee position.
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
    pub const fn mode(self) -> Option<Handle32<FleeEntity>> {
        self.mode
    }

    /// The state byte.
    #[must_use]
    pub const fn state(self) -> u8 {
        self.state
    }

    /// Spawn slot: builds the subtask for the current target when live.
    ///
    /// Restates the verified slot that answers null unless the liveness
    /// gate fires, allocates a fresh block (answering null when none
    /// answers) and otherwise builds the entity form from the mode and
    /// the kind word, or, with a clear mode, the vector form from the
    /// position and the kind word.
    pub fn spawn(
        &self,
        threshold: f32,
        manager: Option<Handle32<TaskMgr>>,
        spawn: &mut impl FleeSpawn,
    ) -> Option<Handle32<SubTask>> {
        if !live_gate(self.flag, self.mode, &self.pos, threshold) {
            return None;
        }
        let block = spawn.alloc(manager)?;
        match self.mode {
            Some(entity) => spawn.construct_entity(block, entity, self.kind),
            None => spawn.construct_vector(block, self.pos, self.kind),
        }
    }

    /// Periodic update slot: keeps the subtask or dispatches past it.
    ///
    /// Restates the verified slot that, when the liveness gate fires,
    /// probes the task and keeps the subtask when the probe's low byte
    /// is set; then runs the task's own state check unless the marks
    /// gate bit is set, keeping the subtask when the check's low byte
    /// is clear and setting the marks pass bit otherwise; and finally
    /// dispatches and answers null.
    pub fn poll(
        &mut self,
        ped: Option<Handle32<FleePed>>,
        threshold: f32,
        poll: &mut impl FleePoll,
    ) -> Option<Handle32<SubTask>> {
        if live_gate(self.flag, self.mode, &self.pos, threshold) && poll.probe(ped) & 0xFF != 0 {
            return self.subtask;
        }
        if self.marks & 1 == 0 {
            if poll.check(ped) & 0xFF == 0 {
                return self.subtask;
            }
            self.marks |= 2;
        }
        poll.dispatch(ped);
        None
    }
}
