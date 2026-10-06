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

/// What the goto asks of its base class and its speed source: the
/// family base constructor and the radius query behind its constructor.
pub trait GotoBase {
    /// Constructs the base task from the caller's word.
    fn construct_base(&mut self, init: u32);

    /// Queries the member radius for the behaviour kind.
    fn query_speed(&mut self, kind: u32) -> f32;
}

/// The child task a picked target is built from (opaque identity).
///
/// Opaque: the lifted picker carries these through the child builders
/// into the combine call, never interprets them. It becomes a real
/// handle when the child classes lift.
#[derive(Debug)]
pub struct GotoChild;

/// What the goto target picker asks of its helpers: the seeder, the two
/// kind hashes, the random source, the goal finder, the allocator, the
/// two child builders and the combiner.
pub trait GotoPick {
    /// Seeds the reaction for the ped.
    fn seed(&mut self, ped: Option<Handle32<GotoPed>>);

    /// Hashes the behaviour kind (called twice, once per hash).
    fn hash_kind(&mut self, kind: u32) -> u32;

    /// Answers a random word for the wait roll.
    fn rand_word(&mut self) -> u32;

    /// Finds the walk goal for the behaviour kind.
    fn find_goal(&mut self, kind: u32);

    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Builds the first child from the task speed and the rate word.
    fn build_first(
        &mut self,
        block: Handle32<UninitTask>,
        speed: f32,
        rate: f32,
    ) -> Option<Handle32<GotoChild>>;

    /// Builds the second child.
    fn build_second(&mut self, block: Handle32<UninitTask>) -> Option<Handle32<GotoChild>>;

    /// Combines both children into the picked task.
    fn combine(
        &mut self,
        block: Handle32<UninitTask>,
        first: Option<Handle32<GotoChild>>,
        second: Option<Handle32<GotoChild>>,
    ) -> Option<Handle32<GotoChild>>;
}

/// The wait roll's random fraction scale (the original's word).
const FRAC_BITS: u32 = 0x3800_0000;

/// The kind hash multiplier (the original's word).
const HASH_MAGIC: u64 = 0x1062_4DD3;

/// The wait's milliseconds per picked unit.
const PER_MILLE: u32 = 1000;

/// The kind hash: the high word of the magic product, shifted down six.
///
/// Restates the original's multiply-then-shift sequence exactly.
const fn hash6(value: u32) -> u32 {
    (((HASH_MAGIC * value as u64) >> 32) as u32) >> 6
}

/// Truncating float-to-int conversion with the original's edge meaning.
///
/// Matches the 32-bit convert instruction bit for bit on every input:
/// NaN, infinities and out-of-range values answer the most negative
/// int, everything else truncates toward zero. (Through the picker the
/// hashes bound the product below the int range, so the edges are
/// unreachable there; the meaning is pinned so the conversion stays
/// exact if that ever changes.)
fn cvtt_ss2si(value: f32) -> i32 {
    if value.is_nan() || value < -2_147_483_648.0 || value >= 2_147_483_648.0 {
        i32::MIN
    } else {
        // In range the host truncation equals the original's.
        value as i32
    }
}

/// Pinned-order float multiply for the wait roll.
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

/// What the goto asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait GotoPool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Copy-constructs a goto from the member word.
    ///
    /// The original passes the member's address; the lift passes the
    /// member word and the proof pins the address.
    fn construct(&mut self, block: Handle32<UninitTask>, member: u32)
    -> Option<Handle32<GotoTask>>;
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

    /// Builds a goto task, constructing its base and querying its speed.
    ///
    /// Restates the verified constructor that runs the family base
    /// constructor on the object from the caller's word, clears the
    /// wait, the stamp, the wait copy and both arming bytes, then
    /// queries the member radius for the kind and keeps it as the
    /// speed. (The original also stamps the class table pointer, which
    /// the proof pins and the lift does not model.) The base-owned
    /// words arrive as arguments: this method sets only what it writes.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn from_init(
        subtask: Option<Handle32<SubTask>>,
        kind: u32,
        pos: [f32; 3],
        flag: bool,
        mode: Option<Handle32<GotoEntity>>,
        init: u32,
        base: &mut impl GotoBase,
    ) -> Self {
        base.construct_base(init);
        let speed = base.query_speed(kind);
        Self {
            subtask,
            kind,
            pos,
            flag,
            mode,
            wait: 0,
            stamp: 0,
            wait_copy: 0,
            armed: false,
            restamp: false,
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

    /// Target picker slot: rolls a wait, then builds the child tasks.
    ///
    /// Restates the verified slot that seeds the reaction, hashes the
    /// kind twice, rolls the wait from a random word over the hash
    /// range, stamps the tick, repeats the wait, arms the timer and
    /// finds the goal, then allocates three blocks: a null first or
    /// second block yields a null child, a null third block answers
    /// null, and otherwise both children combine into the picked task.
    /// The source speed feeds the first child builder beside the rate.
    #[allow(clippy::too_many_arguments)]
    pub fn pick_target(
        &mut self,
        ped: Option<Handle32<GotoPed>>,
        stamp: u32,
        rate: f32,
        manager: Option<Handle32<TaskMgr>>,
        pick: &mut impl GotoPick,
    ) -> Option<Handle32<GotoChild>> {
        pick.seed(ped);
        let first = hash6(pick.hash_kind(self.kind));
        let second = hash6(pick.hash_kind(self.kind));
        let rnd = pick.rand_word();
        let scaled = mul((rnd & 0xFFFF) as f32, f32::from_bits(FRAC_BITS));
        let diff = second.wrapping_sub(first) as i32 as f32;
        let picked = mul(scaled, diff);
        let wait = cvtt_ss2si(picked) as u32;
        let wait = wait.wrapping_add(first).wrapping_mul(PER_MILLE);
        self.wait = wait;
        pick.find_goal(self.kind);
        self.stamp = stamp;
        self.wait_copy = wait;
        self.armed = true;
        let child1 = match pick.alloc(manager) {
            Some(block) => pick.build_first(block, self.speed, rate),
            None => None,
        };
        let child2 = match pick.alloc(manager) {
            Some(block) => pick.build_second(block),
            None => None,
        };
        let Some(block) = pick.alloc(manager) else {
            return None;
        };
        pick.combine(block, child1, child2)
    }
}
