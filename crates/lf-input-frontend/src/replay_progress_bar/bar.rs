//! The replay progress bar object and its three lifted methods.
//!
//! The object owns its words: the inner object it drives, five owned
//! collaborator slots that its teardown releases, the update's mark,
//! total and phase words, and whether it has been torn down. Its
//! collaborators are opaque handles, and every call to them goes through
//! [`ReplayBarWorld`]. The game's globals the update reads and writes are
//! passed in as [`ReplayBarGlobals`], not reached through any state.

use lf_core::Handle32;

use super::world::{BarTag, InnerTag, OwnedTag, Part, ReplayBarWorld};

/// Number of owned collaborator slots the teardown releases.
pub const OWNED_SLOTS: usize = 5;

/// The phase words that skip the second phase of the update.
const PHASE_SKIP_A: u32 = 1;
const PHASE_SKIP_B: u32 = 2;

/// The signal code when the mode and the flag both select the longer
/// signal, and the code otherwise.
const SIGNAL_LONG: u32 = 7;
const SIGNAL_SHORT: u32 = 6;

/// The mode words that select the long signal (together with a set flag).
const MODE_LONG_A: u32 = 2;
const MODE_LONG_B: u32 = 7;

/// Bit OR-ed into the or-word when the signal step runs.
const OR_SIGNAL_BIT: u32 = 2;

/// The shared globals the update reads and writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReplayBarGlobals {
    /// The game mode word, read as a whole word.
    pub mode: u32,
    /// A flag byte, read as a byte.
    pub flag: u8,
    /// The gate byte: when nonzero the gate steps run.
    pub gate: u8,
    /// A word the signal step ORs with [`OR_SIGNAL_BIT`].
    pub or_word: u32,
    /// A word the signal step sets to the compared value.
    pub save_word: u32,
}

/// What the update returns: the inner object itself, or the base update's
/// word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateOutcome {
    /// The inner object, returned when its flag byte is set.
    Inner(Handle32<InnerTag>),
    /// The base update's word, on the path where the inner flag is clear.
    Base(u32),
}

/// The replay progress bar object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayProgressBar {
    /// The handle the collaborators receive as the bar itself.
    pub handle: Handle32<BarTag>,
    /// The inner object the bar drives. Always present: the original
    /// faults when it is null.
    pub inner: Handle32<InnerTag>,
    /// The five owned collaborator slots, `None` when empty.
    pub owned: [Option<Handle32<OwnedTag>>; OWNED_SLOTS],
    /// The update's mark word, set to all ones on every update.
    pub mark: u32,
    /// The update's running total.
    pub total: u32,
    /// The phase word that selects whether the second phase runs.
    pub phase: u32,
    /// Whether the object has been torn down (its dead vtable is in place).
    pub torn: bool,
}

impl ReplayProgressBar {
    /// The teardown: releases the owned slots, closes the token and runs
    /// the finalizers. The base teardown runs first, and the token is
    /// opened before the owned slots are released.
    ///
    /// The first three owned slots are destructed and freed when present.
    /// The last two get one destructor call first; then, only if the slot
    /// still holds a collaborator, a second destructor call and the free.
    /// Every slot is left empty.
    pub fn teardown(&mut self, world: &mut impl ReplayBarWorld) {
        world.base_teardown(self.handle);
        let token = world.open_token();
        world.attach_token(token);
        for slot in &mut self.owned[..3] {
            if let Some(owned) = *slot {
                world.destruct(owned);
                world.free_owned(owned);
                *slot = None;
            }
        }
        for index in 3..OWNED_SLOTS {
            if let Some(owned) = self.owned[index] {
                world.destruct(owned);
                // The slot is read again after the first destructor call.
                if let Some(still) = self.owned[index] {
                    world.destruct(still);
                    world.free_owned(still);
                }
                self.owned[index] = None;
            }
        }
        world.close_token(token);
        world.finalize_a();
        world.finalize_b();
        self.torn = true;
    }

    /// The deleting destructor: runs the teardown, then frees the object's
    /// storage when bit 0 of `flags` is set. Returns the object.
    pub fn deleting_destructor(
        &mut self,
        world: &mut impl ReplayBarWorld,
        flags: u32,
    ) -> Handle32<BarTag> {
        self.teardown(world);
        if flags & 1 != 0 {
            world.release_storage(self.handle);
        }
        self.handle
    }

    /// The update of one frame: a first phase always runs, a second phase
    /// runs unless the phase word is one of the two skip values, and a gate
    /// step and the inner flag decide the result.
    ///
    /// The comparison with the produced word is unsigned. The signal step
    /// runs only when the comparison value is strictly below that word.
    pub fn update(
        &mut self,
        world: &mut impl ReplayBarWorld,
        globals: &mut ReplayBarGlobals,
    ) -> UpdateOutcome {
        world.setup_inner(self.inner, 0);
        let [_, produced_b, produced_c] = world.produce(self.handle);
        self.mark = u32::MAX;
        let (token, mut first, mut second) = world.fill_pair();
        world.consume(self.handle, token, &mut first, &mut second);
        self.total = world.measure().wrapping_add(second);

        if !(self.phase == PHASE_SKIP_A || self.phase == PHASE_SKIP_B) {
            let [_, refreshed_a, refreshed_b] = world.produce(self.handle);
            world.use_a(refreshed_a);
            let used = world.use_b(self.handle, refreshed_a, 0);
            world.use_c(used);
            world.use_d(refreshed_b, used);
            let used_again = world.use_b(self.handle, refreshed_b, 0);
            world.use_e(used_again);
        }

        world.combine(Part::Sub1c, first, second);
        world.step(self.handle);
        let compared = world.compare();
        if compared < produced_b {
            let long_signal =
                (globals.mode == MODE_LONG_A || globals.mode == MODE_LONG_B) && globals.flag != 0;
            world.signal(if long_signal {
                SIGNAL_LONG
            } else {
                SIGNAL_SHORT
            });
            globals.or_word |= OR_SIGNAL_BIT;
            globals.save_word = produced_b;
        }

        world.push_s4(self.handle, produced_c);
        if globals.gate != 0 {
            if !world.inner_flag_set(self.inner) {
                world.gate_a(self.handle);
                world.gate_b(self.handle);
            }
            world.gate_c(self.handle);
        }

        if world.inner_flag_set(self.inner) {
            UpdateOutcome::Inner(self.inner)
        } else {
            world.finish(self.handle);
            UpdateOutcome::Base(world.base_update(self.handle))
        }
    }
}
