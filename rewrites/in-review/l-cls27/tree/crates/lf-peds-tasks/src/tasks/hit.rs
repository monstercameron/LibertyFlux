//! The lifted hit response: one owning type for `CTaskComplexHitResponse`.
//!
//! The original keeps one word of data on this task: the hit kind, which
//! the start slot dispatches on to one of four handlers. This type owns
//! that word as ordinary Rust data and restates each verified 32-bit
//! method with behaviour in it as a method.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-taskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`](crate::tasks::registry).

use lf_core::Handle32;

use crate::tasks::{TaskMgr, UninitTask};

/// A hit-response handler answered by the start slot's lookup (opaque identity).
///
/// Opaque: the lifted task carries it into the case call, never
/// interprets it. It becomes a real handle when its owner lifts.
#[derive(Debug)]
pub struct HitHandler;

/// What the hit response asks of its base class: the family base constructor.
///
/// The constructor runs this before storing the kind. Production code
/// implements it on the lifted base task; tests pass a fake that records
/// the call.
pub trait HitBase {
    /// Constructs the base task.
    fn construct_base(&mut self);
}

/// What the hit response asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait HitPool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Initialises a hit response from the kind word.
    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        kind: u32,
    ) -> Option<Handle32<HitResponse>>;
}

/// What the hit response asks of its handlers: the lookup and the case
/// call behind the start slot.
pub trait HitStart {
    /// Looks up a hit handler from the shared manager.
    fn lookup(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<HitHandler>>;

    /// Runs one case through an answering handler.
    ///
    /// The original calls a different callee per kind; the kind travels
    /// as the argument and the proof pins the kind-to-callee mapping.
    fn run_case(&mut self, handler: Handle32<HitHandler>, kind: u32) -> u32;
}

/// A hit-response task: its hit kind.
///
/// The 32-bit object holds this word past its header; the lifted form
/// owns it directly. Kinds 0 to 3 select a start handler; any other kind
/// answers null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HitResponse {
    /// The hit kind: selects the start handler.
    kind: u32,
}

impl HitResponse {
    /// Builds a hit response, constructing its base first.
    ///
    /// Restates the verified constructor that runs the family base
    /// constructor on the object and then stores the kind word. (The
    /// original also stamps the class table pointer, which the proof
    /// pins and the lift does not model.)
    #[must_use]
    pub fn new(kind: u32, base: &mut impl HitBase) -> Self {
        base.construct_base();
        Self { kind }
    }

    /// The hit kind.
    #[must_use]
    pub const fn kind(self) -> u32 {
        self.kind
    }

    /// Clone slot: allocates a fresh object and initialises a copy.
    ///
    /// Restates the verified slot that asks the pool for a block from
    /// the shared manager and, when one answers, initialises it from
    /// the kind word, answering the initialisation. Answers null when
    /// the allocation fails, without initialising. The source object
    /// is never modified.
    pub fn clone_task(
        &self,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut impl HitPool,
    ) -> Option<Handle32<HitResponse>> {
        let block = pool.alloc(manager)?;
        pool.construct(block, self.kind)
    }

    /// Start slot: dispatches the hit kind to its handler.
    ///
    /// Restates the verified slot that answers 0 for any kind above 3,
    /// looks up a handler from the shared manager (answering 0 when
    /// none answers) and otherwise runs the kind's case, answering that.
    pub fn start(&self, manager: Option<Handle32<TaskMgr>>, run: &mut impl HitStart) -> u32 {
        if self.kind > 3 {
            return 0;
        }
        let Some(handler) = run.lookup(manager) else {
            return 0;
        };
        run.run_case(handler, self.kind)
    }
}
