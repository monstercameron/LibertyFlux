// original: 0x005E8D20 CTaskComplexWanderCop::control_sub_task

/// Return the controller for an active special task, or delegate to the base task.
///
/// `this` is the wander-cop task and `event` provides its navigation owner at
/// `+0x21C`; the owner task kind is at `+0x12C`. When the kind is 2 and a
/// current subtask exists at `this + 0x80`, the method returns the controller
/// pointer stored at `this + 8`. Every other case tail-delegates to the base
/// task's control method with the same object and event. The function pops one
/// incoming stack word.
lf_checker_rt::export!(thiscall, rw_005e8d20(this: u32, event: u32) -> u32 {
    const EVENT_OWNER_OFFSET: u32 = 0x21c;
    const OWNER_TASK_KIND_OFFSET: u32 = 0x12c;
    const CONTROLLER_OFFSET: u32 = 0x08;
    const CURRENT_SUBTASK_OFFSET: u32 = 0x80;
    const SPECIAL_TASK_KIND: u32 = 2;

    let owner = unsafe {
        (event.wrapping_add(EVENT_OWNER_OFFSET) as *const u32).read_unaligned()
    };
    let task_kind = unsafe {
        (owner.wrapping_add(OWNER_TASK_KIND_OFFSET) as *const u32).read_unaligned()
    };
    let subtask = unsafe {
        (this.wrapping_add(CURRENT_SUBTASK_OFFSET) as *const u32).read_unaligned()
    };
    if task_kind == SPECIAL_TASK_KIND && subtask != 0 {
        return unsafe {
            (this.wrapping_add(CONTROLLER_OFFSET) as *const u32).read_unaligned()
        };
    }
    lf_checker_rt::callee_thiscall!(1, u32, this, event)
});
