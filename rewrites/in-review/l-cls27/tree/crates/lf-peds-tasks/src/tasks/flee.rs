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

/// What the flee asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait FleePool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Copy-constructs a flee from the member word, carrying the state byte.
    ///
    /// The original stores the state byte into the clone after the copy
    /// constructor returns; the lift passes it here and the proof pins
    /// the store on the clone blob.
    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        member: u32,
        state: u8,
    ) -> Option<Handle32<FleeTask>>;
}

/// A reaction object built by the flee reaction update (opaque identity).
///
/// Opaque: the lifted update carries it with its marks word, never
/// interprets it. It becomes a real handle when its owner class lifts.
#[derive(Debug)]
pub struct ReactionTask;

/// A built reaction: its identity and its marks word.
///
/// The marks arrive from the builder and the update sets and clears
/// their bits; the proof compares them against the built blob.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reaction {
    /// The built reaction object.
    task: Handle32<ReactionTask>,
    /// Its marks word after the update's bit changes.
    marks: u32,
}

impl Reaction {
    /// Builds a reaction from its identity and marks.
    #[must_use]
    pub const fn new(task: Handle32<ReactionTask>, marks: u32) -> Self {
        Self { task, marks }
    }

    /// The built reaction object.
    #[must_use]
    pub const fn task(self) -> Handle32<ReactionTask> {
        self.task
    }

    /// The marks word.
    #[must_use]
    pub const fn marks(self) -> u32 {
        self.marks
    }
}

/// What the flee reaction update asks of its helpers: the subtask's
/// type slot, the entity's kind word, the probe, the allocator, the
/// reaction builder and the dispatch helper.
pub trait FleeReact {
    /// Answers the subtask's type word through its type slot.
    fn subtask_type(&mut self, sub: Handle32<SubTask>) -> u32;

    /// Answers the entity's kind word.
    fn entity_kind(&mut self, entity: Handle32<FleeEntity>) -> u32;

    /// Probes the ped for the entity (a set low byte refuses the build).
    fn probe(&mut self, ped: Option<Handle32<FleePed>>, entity: Handle32<FleeEntity>) -> u32;

    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Builds the reaction object from the fleeing entity.
    ///
    /// The marks arrive as built; the update changes their bits itself.
    fn build(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
    ) -> Option<Reaction>;

    /// Dispatches the task for its ped after a refused build.
    fn dispatch(&mut self, ped: Option<Handle32<FleePed>>);
}

/// The subtask type the reaction update builds for.
const WANT_TYPE: u32 = 0x16E;

/// The entity kind mask.
const KIND_MASK: u32 = 0x3C0;
/// The wanted entity kind bits.
const WANT_KIND: u32 = 0xC0;

/// The mark bit the update sets on the built reaction.
const MARK_BIT: u32 = 8;

/// The mask clearing the wide-range bit for low kinds.
const WIDE_MASK: u32 = 0xFFFB_FFFF;

/// Kinds below this clear the wide-range bit on the built reaction.
const FLAG_LIMIT: u32 = 0x1B;

/// The flag bit the event handler copies from the state byte.
const FLAG_BIT: u32 = 0x10;

/// The liveness length-squared limit for the event gate (the original's word).
const EVENT_LEN2_BITS: u32 = 0x3D4C_CCCD;

/// The random-word probability scale (the original's word).
const PROB_SCALE_BITS: u32 = 0x3800_0100;

/// The seed probability bound (the original's word).
const PROB_SEED_BITS: u32 = 0x3EA8_F5C3;

/// The spawn probability bound (the original's word).
const PROB_SPAWN_BITS: u32 = 0x3F00_0000;

/// Table entries below this status byte count as fresh.
const STALE_LIMIT: u8 = 3;

/// The seed-path kind bounds (compared signed, like the original).
const SEED_LO: i32 = 0x15;
const SEED_HI: i32 = 0x1B;

/// The spawn-path kind range.
const SPAWN_FIRST: u32 = 0x1B;
const SPAWN_LAST: u32 = 0x1D;

/// The clock refresh step past the base.
const CLOCK_STEP: u32 = 0x4E20;

/// Pinned-order float multiply for the event probabilities.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// The flag bits the event handler sets on a built task: the mark bit
/// set, then the state byte's low bit copied into the flag bit.
fn flag_bits(marks: u32, state: u8) -> u32 {
    let set = marks | MARK_BIT;
    (set & !FLAG_BIT) | ((u32::from(state) & 1) << 4)
}

/// The probe object the event handler checks the entity against
/// (opaque identity).
///
/// Opaque: the lifted handler carries it into the check call, never
/// interprets it. It becomes a real handle when its owner class lifts.
#[derive(Debug)]
pub struct FleeProbe;

/// The fallback target the event handler routes through (opaque identity).
///
/// Opaque for the same reason as [`FleeProbe`].
#[derive(Debug)]
pub struct FleeTarget;

/// A task spawned or routed by the event handler (opaque identity).
///
/// Opaque for the same reason as [`FleeProbe`].
#[derive(Debug)]
pub struct EventChild;

/// The reacting ped's words the event handler reads.
///
/// The original reads these off the ped and its type-table entry; the
/// lift takes them as a snapshot and the proof plants the ped blob and
/// the table behind them. A missing status is the original's null
/// table entry, which faults on the probe.
#[derive(Debug, Clone, Copy)]
pub struct ReactPed {
    /// The reacting ped's identity (for the seed call).
    ped: Handle32<FleePed>,
    /// The type-table entry's status byte.
    status: Option<u8>,
    /// The probe object word.
    probe: Option<Handle32<FleeProbe>>,
    /// The ped flag byte selecting the fallback path.
    flags: u8,
    /// The ped flag byte gating the spawn path.
    aux: u8,
    /// The fallback target word.
    target: Option<Handle32<FleeTarget>>,
}

impl ReactPed {
    /// Builds the ped snapshot from its words.
    #[must_use]
    pub const fn new(
        ped: Handle32<FleePed>,
        status: Option<u8>,
        probe: Option<Handle32<FleeProbe>>,
        flags: u8,
        aux: u8,
        target: Option<Handle32<FleeTarget>>,
    ) -> Self {
        Self {
            ped,
            status,
            probe,
            flags,
            aux,
            target,
        }
    }

    /// The reacting ped's identity.
    #[must_use]
    pub const fn ped(self) -> Handle32<FleePed> {
        self.ped
    }

    /// The status byte (`None` is the original's null entry).
    #[must_use]
    pub const fn status(self) -> Option<u8> {
        self.status
    }

    /// The probe object word (`None` is the original's null).
    #[must_use]
    pub const fn probe(self) -> Option<Handle32<FleeProbe>> {
        self.probe
    }

    /// The ped flag byte.
    #[must_use]
    pub const fn flags(self) -> u8 {
        self.flags
    }

    /// The ped auxiliary flag byte.
    #[must_use]
    pub const fn aux(self) -> u8 {
        self.aux
    }

    /// The fallback target (`None` is the original's null).
    #[must_use]
    pub const fn target(self) -> Option<Handle32<FleeTarget>> {
        self.target
    }
}

/// What the flee event handler answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FleeAnswer {
    /// The cooldown spawn path's task.
    Spawned(Handle32<EventChild>),
    /// A built task with its flag bits set (task A and task B share
    /// this shape; the marks offset is pinned per path by the proof).
    Built(Reaction),
    /// The fallback target path's task.
    Routed(Handle32<EventChild>),
}

/// What the flee event handler asks of its helpers: the random source,
/// the seeder, the entity kind word, the state check, the allocator,
/// the spawn builder and the three task builders.
pub trait FleeEvent {
    /// Answers a random word for the seed and spawn rolls.
    fn rand_word(&mut self) -> u32;

    /// Seeds the reaction for the ped from the seed argument.
    fn seed(&mut self, ped: Handle32<FleePed>, seed_arg: u32);

    /// Answers the entity's kind word.
    fn entity_kind(&mut self, entity: Handle32<FleeEntity>) -> u32;

    /// Runs the state check for the probe object and the entity (a
    /// clear low byte takes the build path).
    fn check(&mut self, probe: Option<Handle32<FleeProbe>>, entity: Handle32<FleeEntity>) -> u32;

    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Spawns the cooldown task from the fleeing entity.
    fn spawn(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
    ) -> Option<Handle32<EventChild>>;

    /// Builds task A from the fleeing entity.
    ///
    /// The marks arrive as built; the handler sets their bits itself.
    fn build_a(
        &mut self,
        block: Handle32<UninitTask>,
        entity: Handle32<FleeEntity>,
    ) -> Option<Reaction>;

    /// Builds task B from the flee position and the rate word.
    ///
    /// The marks arrive as built; the handler sets their bits itself.
    fn build_b(
        &mut self,
        block: Handle32<UninitTask>,
        pos: [f32; 3],
        rate: u32,
    ) -> Option<Reaction>;

    /// Builds the fallback task from the target and the flee position.
    fn build_c(
        &mut self,
        block: Handle32<UninitTask>,
        target: Handle32<FleeTarget>,
        pos: [f32; 3],
    ) -> Option<Handle32<EventChild>>;
}

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

    /// Clone slot: allocates a fresh object, constructs a copy, carries the state.
    ///
    /// Restates the verified slot that asks the pool for a block from
    /// the shared manager, copy-constructs it from the embedded member
    /// and carries the state byte into the clone, answering the
    /// construction. Panics when no block answers, where the original
    /// faults storing the state byte through the null clone. The source
    /// object is never modified.
    pub fn clone_task(
        &self,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut impl FleePool,
    ) -> Option<Handle32<FleeTask>> {
        let Some(block) = pool.alloc(manager) else {
            panic!("flee clone without a block: the original faults storing the state byte");
        };
        pool.construct(block, self.kind, self.state)
    }

    /// Reaction update slot: builds the reaction or dispatches past it.
    ///
    /// Restates the verified slot that dispatches and answers null
    /// unless every gate passes: the subtask's type word must be the
    /// wanted type, the mode entity must be set with the wanted kind
    /// bits, and the probe's low byte must refuse. Past the gates it
    /// allocates and builds the reaction, sets its mark bit, and clears
    /// its wide-range bit for kinds below the flag limit. Panics
    /// without a subtask, without a ped on the probe path, and without
    /// a block or a build, where the original faults on each.
    pub fn react(
        &self,
        ped: Option<Handle32<FleePed>>,
        manager: Option<Handle32<TaskMgr>>,
        react: &mut impl FleeReact,
    ) -> Option<Reaction> {
        let Some(sub) = self.subtask else {
            panic!("flee react without a subtask: the original faults reading its table");
        };
        if react.subtask_type(sub) != WANT_TYPE {
            react.dispatch(ped);
            return None;
        }
        let Some(entity) = self.mode else {
            react.dispatch(ped);
            return None;
        };
        if react.entity_kind(entity) & KIND_MASK != WANT_KIND {
            react.dispatch(ped);
            return None;
        }
        if ped.is_none() {
            panic!("flee react without a ped: the original faults reading the probe word");
        }
        if react.probe(ped, entity) & 0xFF != 0 {
            react.dispatch(ped);
            return None;
        }
        let Some(block) = react.alloc(manager) else {
            panic!("flee react without a block: the original faults marking the null build");
        };
        let Some(built) = react.build(block, entity) else {
            panic!("flee react without a build: the original faults marking the null build");
        };
        let mut marks = built.marks() | MARK_BIT;
        if self.kind < FLAG_LIMIT {
            marks &= WIDE_MASK;
        }
        Some(Reaction::new(built.task(), marks))
    }

    /// Event handler slot: seeds a reaction, then builds a flee task,
    /// a fallback task, or nothing.
    ///
    /// Restates the verified slot. Past the liveness gate it probes the
    /// ped type's freshness and seeds the reaction for fresh entries
    /// (kinds past the seed bound unconditionally, kinds inside it on a
    /// random roll). Past the entity kind gates and a passed state
    /// check it either refreshes the clock and spawns the cooldown task
    /// (spawn kinds, right flags, clock below base, a second roll won)
    /// or builds task A with its flag bits; otherwise the fallback
    /// routes through the ped target or builds task B with its flag
    /// bits. Panics without a ped, without a table entry, and without
    /// a block or a build on the flag-bit paths, where the original
    /// faults on each.
    #[allow(clippy::too_many_arguments)]
    pub fn handle_event(
        &self,
        ped: Option<ReactPed>,
        seed_arg: u32,
        clock: &mut u32,
        clock_base: u32,
        rate: u32,
        manager: Option<Handle32<TaskMgr>>,
        event: &mut impl FleeEvent,
    ) -> Option<FleeAnswer> {
        if !live_gate(
            self.flag,
            self.mode,
            &self.pos,
            f32::from_bits(EVENT_LEN2_BITS),
        ) {
            return None;
        }
        let Some(ped) = ped else {
            panic!("flee event without a ped: the original faults reading its type");
        };
        let Some(status) = ped.status() else {
            panic!("flee event without a table entry: the original faults probing it");
        };
        let fresh = status < STALE_LIMIT;
        let kind_i = self.kind as i32;
        let mut do_seed = false;
        if fresh {
            if kind_i >= SEED_HI {
                do_seed = true;
            } else if kind_i >= SEED_LO {
                let r = event.rand_word();
                if f32::from_bits(PROB_SEED_BITS)
                    > mul((r as i32) as f32, f32::from_bits(PROB_SCALE_BITS))
                {
                    do_seed = true;
                }
            }
        }
        if do_seed {
            event.seed(ped.ped(), seed_arg);
        }
        let stage_two = match self.mode {
            Some(entity)
                if event.entity_kind(entity) & KIND_MASK == WANT_KIND
                    && event.check(ped.probe(), entity) & 0xFF == 0 =>
            {
                true
            }
            _ => false,
        };
        if stage_two {
            let entity = self.mode.expect("stage two keeps an entity");
            if (SPAWN_FIRST..=SPAWN_LAST).contains(&self.kind)
                && ped.flags() & 4 == 0
                && *clock < clock_base
                && ped.aux() & 4 != 0
            {
                let r2 = event.rand_word();
                if f32::from_bits(PROB_SPAWN_BITS)
                    > mul((r2 as i32) as f32, f32::from_bits(PROB_SCALE_BITS))
                {
                    *clock = clock_base.wrapping_add(CLOCK_STEP);
                    let Some(block) = event.alloc(manager) else {
                        return None;
                    };
                    return event.spawn(block, entity).map(FleeAnswer::Spawned);
                }
            }
            let Some(block) = event.alloc(manager) else {
                panic!("flee event without a block: the original faults flagging the null build");
            };
            let Some(built) = event.build_a(block, entity) else {
                panic!("flee event without a build: the original faults flagging the null build");
            };
            let mut marks = flag_bits(built.marks(), self.state);
            if kind_i < FLAG_LIMIT as i32 {
                marks &= WIDE_MASK;
            }
            return Some(FleeAnswer::Built(Reaction::new(built.task(), marks)));
        }
        if ped.flags() & 4 != 0 {
            let Some(target) = ped.target() else {
                return None;
            };
            let Some(block) = event.alloc(manager) else {
                return None;
            };
            return event
                .build_c(block, target, self.pos)
                .map(FleeAnswer::Routed);
        }
        let Some(block) = event.alloc(manager) else {
            panic!("flee event without a block: the original faults flagging the null build");
        };
        let Some(built) = event.build_b(block, self.pos, rate) else {
            panic!("flee event without a build: the original faults flagging the null build");
        };
        let marks = flag_bits(built.marks(), self.state);
        Some(FleeAnswer::Built(Reaction::new(built.task(), marks)))
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
