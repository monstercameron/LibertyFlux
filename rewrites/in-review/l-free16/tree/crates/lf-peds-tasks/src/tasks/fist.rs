//! The lifted fist shake: one owning type for `CTaskSimpleShakeFist`.
//!
//! The original keeps a small task per shaking fist: a held-object slot
//! the event slot damps and releases, and a second member word the clone
//! slot copies into the copy constructor. This type owns those two words
//! as ordinary Rust data (opaque identities, never addresses) and restates
//! each verified 32-bit method with behaviour in it as a method.
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

/// The object a fist shake holds (its held slot).
///
/// Opaque: the lifted task tests it for null, hands it to the damp and
/// release calls and clears it, never interprets it. It becomes a real
/// handle when its owner class lifts.
#[derive(Debug)]
pub struct FistHeld;

/// The second member word the clone slot copies (opaque identity).
///
/// Opaque for the same reason as [`FistHeld`].
#[derive(Debug)]
pub struct FistLink;

/// The damp rate word the event slot pushes with its damp call, as bits.
///
/// The original pushes this fixed word (`-4.0` as a float) beside the
/// held object; the lift passes it as the rate argument and the proof
/// pins these bits on the call.
pub const DAMP_RATE_BITS: u32 = 0xC080_0000;

/// What the fist shake asks of its held object: the target-side collaborator.
///
/// The event slot damps the held object on every event and, on the two
/// taken events, releases it and clears the slot. Production code
/// implements this on the lifted target; tests pass a fake that records
/// calls and scripts answers.
pub trait FistTarget {
    /// Damps the held object at the given rate.
    ///
    /// Takes the slot by mutable reference because the original re-reads
    /// the slot after this call: the damp may release it, in which case
    /// the release below is skipped.
    fn damp(&mut self, slot: &mut Option<Handle32<FistHeld>>, rate: f32);

    /// Releases the held object from its task.
    fn release(&mut self, held: Handle32<FistHeld>);
}

/// What the fist shake asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait FistPool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Copy-constructs a fist shake from the member word.
    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        link: Option<Handle32<FistLink>>,
    ) -> Option<Handle32<ShakeFist>>;
}

/// A fist-shake task: its held object and its member word.
///
/// The 32-bit object holds these two words at fixed offsets past its
/// header; the lifted form owns them directly. A missing entry (`None`)
/// is the original's null word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShakeFist {
    /// The held-object slot: damped and released by the event slot.
    held: Option<Handle32<FistHeld>>,
    /// The member word: copied into the copy constructor by the clone slot.
    link: Option<Handle32<FistLink>>,
}

impl ShakeFist {
    /// Builds a fist shake from its two words.
    #[must_use]
    pub const fn new(held: Option<Handle32<FistHeld>>, link: Option<Handle32<FistLink>>) -> Self {
        Self { held, link }
    }

    /// The held-object slot (`None` is the original's null).
    #[must_use]
    pub const fn held(self) -> Option<Handle32<FistHeld>> {
        self.held
    }

    /// The member word (`None` is the original's null).
    #[must_use]
    pub const fn link(self) -> Option<Handle32<FistLink>> {
        self.link
    }

    /// Clone slot: allocates a fresh object and constructs a copy.
    ///
    /// Restates the verified slot that asks the pool for a block from
    /// the shared manager and, when one answers, copy-constructs it
    /// from the member word, answering the construction either way.
    /// Answers null when the allocation fails, without constructing.
    /// The source object is never modified.
    pub fn clone_task(
        &self,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut impl FistPool,
    ) -> Option<Handle32<ShakeFist>> {
        let block = pool.alloc(manager)?;
        pool.construct(block, self.link)
    }

    /// Event slot: damps the held object, releases it on events 1 and 2.
    ///
    /// Restates the verified slot that damps the held slot at
    /// [`DAMP_RATE_BITS`] whenever it is set, then, on events 1 and 2,
    /// re-reads the slot and, when still set, releases it and clears
    /// the slot, answering 1. Any other event only damps and answers 0.
    pub fn handle_event(&mut self, event: u32, target: &mut impl FistTarget) -> u32 {
        if event == 1 || event == 2 {
            if self.held.is_some() {
                target.damp(&mut self.held, f32::from_bits(DAMP_RATE_BITS));
            }
            if let Some(held) = self.held {
                target.release(held);
                self.held = None;
            }
            1
        } else {
            if self.held.is_some() {
                target.damp(&mut self.held, f32::from_bits(DAMP_RATE_BITS));
            }
            0
        }
    }
}
