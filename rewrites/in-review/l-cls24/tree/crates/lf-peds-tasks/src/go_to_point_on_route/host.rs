//! What the go-to-point-on-route task asks of the outside world.
//!
//! Every call a verified method makes that is not itself lifted is one
//! method here. Opaque handles (`u32` words the host assigns) name the new
//! task; the lift never reads through them.

use super::task::GoToPointOnRouteTask;

/// The offset of a block inside the task that a constructor is handed by
/// address. The verified rewrite passes the address of a field block of the
/// source task; the lift names the block by its offset from the start of
/// the task, so the host can resolve it for whichever object it holds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskField(pub u32);

/// The arguments the class constructor receives when a task is copied, in
/// the order the verified clone pushes them. The constructor's first
/// argument is the new object itself and is not part of this record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CopyArgs {
    /// The progress word, copied as raw bits.
    pub progress_bits: u32,
    /// The block at offset `0x40` of the source task.
    pub block_40: TaskField,
    /// The block at offset `0xD0` of the source task.
    pub block_d0: TaskField,
    /// The active byte of the source task, zero-extended.
    pub active: u32,
    /// The word at offset `0xA0` of the source task.
    pub word_a0: u32,
    /// Bit 0 of the source task's flag word at offset `0xC4`.
    pub flag_bit_0: u32,
    /// Bit 1 of the source task's flag word at offset `0xC4`.
    pub flag_bit_1: u32,
}

/// The calls the go-to-point-on-route task makes on the task system.
pub trait GoToPointOnRouteHost {
    /// The route refresh: the task system's route updater, called on the
    /// task with one word. The callee may write any part of the task, and
    /// the caller reads the task's flag word after the call returns. The
    /// answer is not used by the verified method.
    fn refresh_route(&mut self, task: &mut GoToPointOnRouteTask, arg: u32);

    /// Allocates storage for a new task of this class. Returns the new
    /// task's handle, or 0 when the allocator has none to give.
    fn allocate_task(&mut self) -> u32;

    /// The class constructor, run on a new task with the copy arguments.
    /// Returns the constructor's answer, which the copy returns in turn.
    fn construct_copy(&mut self, new_task: u32, args: &CopyArgs) -> u32;
}
