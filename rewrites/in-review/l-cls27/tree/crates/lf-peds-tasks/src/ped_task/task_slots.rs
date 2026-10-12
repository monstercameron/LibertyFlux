//! The task manager's slot arrays: five task pointers, then six more, and
//! the requests that the first bomb slots make of the task interface.
//!
//! The original reads these words straight out of the manager object: five
//! consecutive pointers at offset zero, six consecutive pointers at offset
//! 0x14. This module owns them as [`TaskSlots`]. The task interface the
//! requests call, and the three methods the sequence calls, travel as
//! [`ManagerCallees`]. The manager's other parts (its embedded task record
//! and its update source) are not in this module.

/// Number of pointers in the first array.
pub const PRIMARY_COUNT: usize = 5;

/// Number of pointers in the second array.
pub const SECONDARY_COUNT: usize = 6;

/// Index in the primary array of the first bomb task pointer.
pub const FIRST_BOMB_SLOT: usize = 1;

/// Index in the primary array of the second bomb task pointer.
pub const SECOND_BOMB_SLOT: usize = 2;

/// The task kind the first bomb request asks for.
pub const FIRST_BOMB_KIND: u32 = 1;

/// The task kind the second bomb request asks for.
pub const SECOND_BOMB_KIND: u32 = 2;

/// The task manager's pointer arrays, owned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskSlots {
    /// Five task pointers at offset zero.
    pub primary: [u32; PRIMARY_COUNT],
    /// Six task pointers at offset 0x14, directly after the primary array.
    pub secondary: [u32; SECONDARY_COUNT],
}

/// The task interface and the manager's own preparatory methods. Each
/// travels as a method, called in the same order as the original.
pub trait ManagerCallees {
    /// The task interface's request for a bomb task of `kind`. The original
    /// passes the manager, two zero words and the kind, and the proof
    /// compares all four.
    fn request_task(&mut self, kind: u32);
    /// The first preparatory method of the manager.
    fn prepare_tasks(&mut self);
    /// The second preparatory method of the manager.
    fn refresh_tasks(&mut self);
    /// The final update method; its answer is the sequence's answer.
    fn final_update(&mut self) -> u32;
}

impl TaskSlots {
    /// The index of the first nonnull primary pointer, or the primary count
    /// when all five are null.
    #[must_use]
    pub fn first_occupied(&self) -> usize {
        self.primary
            .iter()
            .position(|&task| task != 0)
            .unwrap_or(PRIMARY_COUNT)
    }

    /// Whether one of the six secondary pointers equals `needle`.
    #[must_use]
    pub fn contains(&self, needle: u32) -> bool {
        self.secondary.contains(&needle)
    }

    /// Requests the first applicable bomb task: kind one when the first bomb
    /// slot is nonnull, otherwise kind two when the second is. No request is
    /// made when both are null.
    pub fn request_bomb_task<C: ManagerCallees>(&self, callees: &mut C) {
        if self.primary[FIRST_BOMB_SLOT] != 0 {
            callees.request_task(FIRST_BOMB_KIND);
        } else if self.primary[SECOND_BOMB_SLOT] != 0 {
            callees.request_task(SECOND_BOMB_KIND);
        }
    }

    /// Requests a kind-one bomb task when the first bomb slot is nonnull.
    pub fn request_first_bomb_task<C: ManagerCallees>(&self, callees: &mut C) {
        if self.primary[FIRST_BOMB_SLOT] != 0 {
            callees.request_task(FIRST_BOMB_KIND);
        }
    }

    /// Runs the two preparatory methods in order, then the final update,
    /// and returns the final update's answer.
    pub fn update_sequence<C: ManagerCallees>(&self, callees: &mut C) -> u32 {
        callees.prepare_tasks();
        callees.refresh_tasks();
        callees.final_update()
    }
}
