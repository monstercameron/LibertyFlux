//! The move-between-points scenario task: the scenario's own methods.
//!
//! A pedestrian walks a route between scenario points. The scenario task holds
//! a status word, an activity byte, a guard word, a state word and a payload
//! handle, and its methods build a copy of themselves, release the payload,
//! run a sub-task each tick, and report a blend weight.
//!
//! Field names follow the verified rewrites where they agree. Where two of them
//! give the same word different meanings, the field is named for the rewrite
//! that the method belongs to and the registry says so.

use super::host::{MoveBetweenPointsHost, SubTask};

/// Status bit 0: a sticky stop (the scenario does not run its sub-operation again).
pub const STATUS_STICKY: u32 = 0x01;
/// Status bit 1: the sub-operation has been accepted.
pub const STATUS_DONE: u32 = 0x02;
/// Blend weight of a scenario that is running or about to run.
pub const ACTIVE_WEIGHT: f32 = 25.0;
/// Blend weight of an idle scenario.
pub const IDLE_WEIGHT: f32 = -1.0;

/// The scenario task's own data, as the lifted methods read and write it.
///
/// Only the words the lifted methods touch are here. The sub-task is not a
/// field: the methods that run it take it as an argument (see
/// [`MoveBetweenPointsScenario::run_tick`]).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MoveBetweenPointsScenario {
    /// Status bits. Bit 0 is sticky, bit 1 is done.
    pub status: u32,
    /// The auxiliary identifier. It selects the chat variant in the full
    /// scenario, and is copied to a new scenario by [`Self::clone_scenario`].
    pub aux_id: u32,
    /// A second auxiliary word, copied by [`Self::clone_scenario`].
    pub aux_word: u32,
    /// The guard word. The scenario counts as idle only when this is zero and
    /// the activity byte is set.
    pub pending_guard: u32,
    /// The activity byte. Non-zero when the scenario is active.
    pub active: u8,
    /// The state word. Zero and one are the walking states; the scenario moves
    /// through them as it commits a destination and a leg.
    pub state: u32,
    /// The payload handle held while the scenario is in its move state. Zero
    /// when nothing is held.
    pub payload: u32,
}

impl MoveBetweenPointsScenario {
    /// The scenario's blend weight for the frame.
    ///
    /// Returns [`ACTIVE_WEIGHT`] at once when bit 1 of the status is set.
    /// Otherwise the scenario is idle (weight [`IDLE_WEIGHT`]) when its activity
    /// byte is set and its guard word is zero. Failing that, the weight is
    /// active when the state is zero or one and idle for any other state.
    #[must_use]
    pub fn blend_weight(&self) -> f32 {
        if (self.status >> 1) & 1 != 0 {
            return ACTIVE_WEIGHT;
        }
        if self.active != 0 && self.pending_guard == 0 {
            return IDLE_WEIGHT;
        }
        if self.state == 0 || self.state == 1 {
            ACTIVE_WEIGHT
        } else {
            IDLE_WEIGHT
        }
    }

    /// The deleting destructor: runs the base class destructor on the scenario,
    /// then, when bit 0 of `flags` is set, frees it through the heap.
    ///
    /// The lift does not drop the Rust value; the owner that calls this drops
    /// it, because the free is a call the host makes. The order of the
    /// destructor, the heap read and the free is the original's.
    pub fn deleting_destructor<H: MoveBetweenPointsHost>(&mut self, host: &mut H, flags: u32) {
        host.class_destructor(self);
        if flags & 1 != 0 {
            let heap = host.heap_singleton();
            host.heap_free(heap, self);
        }
    }

    /// The clone factory: allocates a slot from the heap and constructs a new
    /// scenario in it from this one's `aux_id`, `aux_word` and `pending_guard`.
    ///
    /// Returns zero when the allocation fails, and otherwise the word the
    /// constructor returns. The three words are read after the allocation, as
    /// the original reads them.
    pub fn clone_scenario<H: MoveBetweenPointsHost>(&self, host: &mut H) -> u32 {
        let heap = host.heap_singleton();
        let slot = host.heap_alloc(heap);
        if slot == 0 {
            return 0;
        }
        host.construct(slot, self.aux_id, self.aux_word, self.pending_guard)
    }

    /// One tick of the scenario's sub-task.
    ///
    /// Unless the sub-task's latch already has bit 0 set, the sub-task runs with
    /// `(a1, a2, a3)`. A zero low byte from the run ends the tick with `false`.
    /// Otherwise bit 1 of the sub-task's latch is set. Then, when the state is
    /// one, it is reset to zero and a non-zero payload is released through the
    /// host and cleared. The tick ends with `true`.
    ///
    /// The verified rewrite reads the sub-task from the scenario's own sub-task
    /// slot. The lift takes the sub-task as an argument: the caller holds the same
    /// object, and the proof passes the object the rewrite reads.
    pub fn run_tick<S: SubTask, H: MoveBetweenPointsHost>(
        &mut self,
        sub: &mut S,
        host: &mut H,
        a1: u32,
        a2: u32,
        a3: u32,
    ) -> bool {
        if sub.latch() & 1 == 0 {
            if sub.run(a1, a2, a3) & 0xff == 0 {
                return false;
            }
            let latch = sub.latch();
            sub.set_latch(latch | 2);
        }
        if self.state == 1 {
            self.state = 0;
            let payload = self.payload;
            if payload != 0 {
                host.release_payload(payload);
            }
            self.payload = 0;
        }
        true
    }
}
