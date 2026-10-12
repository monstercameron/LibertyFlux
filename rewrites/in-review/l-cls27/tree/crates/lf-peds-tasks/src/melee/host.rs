//! What the complex melee task asks of the outside world.
//!
//! Every call a verified method makes that is not itself lifted is one
//! method here. The two words of global state each of the constructor and
//! the initialise method copies are read by the host and handed over as
//! values. Opaque handles ([`Handle32`]) name the sub-task; the lift never
//! reads through them.

use lf_core::boundary::Handle32;

use super::task::{MeleeTask, StarterHandleTag, SubTask, TaskManagerTag};

/// The arguments of the constructor, in the order the verified constructor
/// receives them, with the two words of the storage it is built in that it
/// carries over instead of writing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CtorArgs {
    /// The word stored at offset `0x30`.
    pub initial_word: u32,
    /// The second flag word; its low bit becomes bit 1 of the flag byte.
    pub flag_two: u32,
    /// The third flag word; its low bit becomes bit 2 of the flag byte.
    pub flag_three: u32,
    /// The fourth flag word; its low bit becomes bit 0 of the flag byte.
    pub flag_four: u32,
    /// The fifth flag word; its low bit becomes bit 3 of the flag byte.
    pub flag_five: u32,
    /// The first word forwarded to the target-node setter.
    pub target_first: u32,
    /// The second word forwarded to the target-node setter.
    pub target_second: u32,
    /// The sub-task word of the storage the task is built in. No verified
    /// constructor writes it, so the caller carries it in.
    pub sub_task: Option<Handle32<SubTask>>,
    /// The byte at offset `0xF1` of the storage the task is built in. The
    /// constructor keeps its high nibble and clears its low nibble.
    pub incoming_flags_high: u8,
}

/// The two words of global state the constructor copies into the task.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StampWords {
    /// The word stored at offset `0x38`.
    pub word_38: u32,
    /// The word stored at offset `0x3C`.
    pub word_3c: u32,
}

/// The two words of global state the initialise method copies into the task.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InitWords {
    /// The word stored at offset `0xDC`.
    pub word_dc: u32,
    /// The word stored at offset `0xE0`.
    pub word_e0: u32,
}

/// The arguments of the melee starter call, in the order the start method
/// passes them. The flag bits keep the original's order: bit 1, bit 2,
/// bit 0, bit 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartArgs {
    /// The opaque handle the task manager answered.
    pub handle: Handle32<StarterHandleTag>,
    /// The task's word at offset `0x30`, read after the mode calls.
    pub mode_word: u32,
    /// Bit 1 of the flag byte.
    pub flag_bit1: u32,
    /// Bit 2 of the flag byte.
    pub flag_bit2: u32,
    /// Bit 0 of the flag byte.
    pub flag_bit0: u32,
    /// Bit 3 of the flag byte.
    pub flag_bit3: u32,
    /// The answer of the flag helper.
    pub computed: u32,
}

/// The calls the complex melee task makes on the task system.
pub trait MeleeHost {
    /// The base class constructor, run on the task first. It may write any
    /// part of the task; the verified proof observes no write.
    fn construct_base(&mut self, task: &mut MeleeTask);

    /// The sub-object reset, run on one of the ten slots named by
    /// [`crate::melee::task::SLOT_OFFSETS`]. The offset is from the start of
    /// the task. The reset owns the slot's twelve bytes.
    fn reset_slot(&mut self, task: &mut MeleeTask, offset: u32);

    /// The target-node setter, run on the task with the last two
    /// constructor arguments, in order. Its answer is not used.
    fn set_target_node(&mut self, task: &mut MeleeTask, first: u32, second: u32);

    /// The two words of global state the constructor copies into the task.
    /// Read after the target-node setter runs.
    fn stamp_words(&mut self) -> StampWords;

    /// The destructor: run by the destroy method on the task, always.
    fn destroy_task(&mut self, task: &mut MeleeTask);

    /// The task allocator's opaque manager handle, read from global state.
    /// The destroy method reads it only when the flags ask for the task to
    /// be freed.
    fn manager(&mut self) -> Option<Handle32<TaskManagerTag>>;

    /// The task allocator's free helper, run with the manager handle. The
    /// freed task is the one being destroyed.
    fn free_task(&mut self, manager: Option<Handle32<TaskManagerTag>>);

    /// The task manager's handle for the melee starter, asked of the
    /// manager handle. A null answer ends the start method quietly.
    fn task_handle(
        &mut self,
        manager: Option<Handle32<TaskManagerTag>>,
    ) -> Option<Handle32<StarterHandleTag>>;

    /// The mode helper, run on the task. Only the low byte of its answer
    /// reaches the flag helper.
    fn mode_value(&mut self, task: &mut MeleeTask) -> u32;

    /// The flag helper, run on the task with the low byte of the mode
    /// answer. The callee may write any part of the task; the start method
    /// reads the task's word at offset `0x30` after it returns.
    fn flag_value(&mut self, task: &mut MeleeTask, mode_byte: u32) -> u32;

    /// The melee starter, run with the handle, the mode word, the four
    /// flag bits and the flag helper's answer. Its answer is not used.
    fn start_melee(&mut self, args: &StartArgs);

    /// The two words of global state the initialise method copies into the
    /// task.
    fn timing_words(&mut self) -> InitWords;

    /// The target setup, run on the task with the initialise argument.
    fn setup_target(&mut self, task: &mut MeleeTask, arg: u32);

    /// The registration helper, run on the task with the initialise
    /// argument, the kind tag and the constant 1. Its answer is not used.
    fn register(&mut self, task: &mut MeleeTask, arg: u32, kind: u32, flag: u32);

    /// The reference cleanup, run on the task first by the follow-up
    /// method. The callee may write any part of the task; the method reads
    /// the task's flag byte after it returns.
    fn release_refs(&mut self, task: &mut MeleeTask);

    /// The sub-task's query through its virtual slot: the follow-up method
    /// asks it of the sub-task's handle and continues only on the two
    /// accepted answers.
    fn subtask_query(&mut self, sub: Option<Handle32<SubTask>>) -> u32;

    /// The follow-up sink, run on the task with the argument, the kind tag
    /// and 0. Its answer is the follow-up method's answer.
    fn follow_up_sink(&mut self, task: &mut MeleeTask, arg: u32, kind: u32, zero: u32) -> u32;
}
