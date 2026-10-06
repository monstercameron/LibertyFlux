//! Adopting an orphan slot: owner generation, hook and marker.
//!
//! Lifted from the verified rewrite. The slot record carries its owner
//! word at `+0x70`; the rest of the record is not modeled. An owner of
//! all-ones means orphaned.

/// Owner value meaning the slot is orphaned.
const ORPHAN: u32 = 0xffff_ffff;

/// One adoptable slot: the owner word at `+0x70`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdoptSlot {
    /// The owner word: all-ones when orphaned.
    pub owner: u32,
}

impl AdoptSlot {
    /// Builds a slot with the given owner word.
    #[must_use]
    pub const fn new(owner: u32) -> Self {
        Self { owner }
    }

    /// Adopts an orphan slot: records its owner, notifies, marks it.
    ///
    /// Restates `stream_slot_adopt`: when the owner is not all-ones the
    /// slot is already adopted and nothing happens; otherwise `generation` is
    /// stored as the owner, the hook runs, the slot is marked live
    /// through the marker, and the marker's answer is the outcome. The
    /// two address answers narrow: the owner field's address becomes
    /// [`AdoptOutcome::AlreadyAdopted`], and the marker's answer (an
    /// address the marker owns) travels as an opaque word.
    pub fn adopt<H: AdoptHook, M: MarkSlot>(
        &mut self,
        generation: u32,
        hook: &mut H,
        marker: &mut M,
    ) -> AdoptOutcome {
        if self.owner != ORPHAN {
            return AdoptOutcome::AlreadyAdopted;
        }
        self.owner = generation;
        hook.adopted(generation);
        AdoptOutcome::Marked(marker.mark_live(self))
    }
}

/// Outcome of adopting a slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdoptOutcome {
    /// The slot was already adopted; nothing happened.
    AlreadyAdopted,
    /// The slot was marked; the marker's opaque answer.
    Marked(u32),
}

/// Runs when a slot's owner is recorded: the adoption hook.
pub trait AdoptHook {
    /// Records the adoption of the slot now owned by `generation`.
    fn adopted(&mut self, generation: u32);
}

/// Marks an adopted slot live: the adoption marker.
pub trait MarkSlot {
    /// Marks `slot` live, answering the marker's word.
    fn mark_live(&mut self, slot: &AdoptSlot) -> u32;
}

impl<F: FnMut(u32)> AdoptHook for F {
    fn adopted(&mut self, generation: u32) {
        self(generation);
    }
}

impl<F: FnMut(&AdoptSlot) -> u32> MarkSlot for F {
    fn mark_live(&mut self, slot: &AdoptSlot) -> u32 {
        self(slot)
    }
}
