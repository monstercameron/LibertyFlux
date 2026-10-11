// original: 0x005E8CB0 CTaskComplexWanderCop::create_next_sub_task

/// Select the next wander-cop subtask after comparing its state with the controller.
///
/// `this` is the wander-cop task and `event` provides the owner and route
/// records. A non-special owner delegates to the base task. For the special
/// task kind, an active route returns zero. Otherwise, a present current
/// subtask and the controller at `this + 8` each provide a state through
/// virtual slot 3. Equal states end the sequence with zero; different states
/// ask the current subtask's virtual slot 1 for its next task. A missing
/// subtask delegates to the base task. All delegated or virtual return values
/// pass through unchanged, and the function pops one incoming stack word.
lf_checker_rt::export!(thiscall, rw_005e8cb0(this: u32, event: u32) -> u32 {
    const EVENT_OWNER_OFFSET: u32 = 0x21c;
    const OWNER_TASK_KIND_OFFSET: u32 = 0x12c;
    const EVENT_ROUTE_OFFSET: u32 = 0x22c;
    const ROUTE_ACTIVE_BYTE_OFFSET: u32 = 0x08;
    const CONTROLLER_OFFSET: u32 = 0x08;
    const CURRENT_SUBTASK_OFFSET: u32 = 0x80;
    const SPECIAL_TASK_KIND: u32 = 2;

    let owner = unsafe {
        (event.wrapping_add(EVENT_OWNER_OFFSET) as *const u32).read_unaligned()
    };
    let task_kind = unsafe {
        (owner.wrapping_add(OWNER_TASK_KIND_OFFSET) as *const u32).read_unaligned()
    };
    if task_kind != SPECIAL_TASK_KIND {
        return lf_checker_rt::callee_thiscall!(4, u32, this, event);
    }

    let route = unsafe {
        (event.wrapping_add(EVENT_ROUTE_OFFSET) as *const u32).read_unaligned()
    };
    let route_active = unsafe {
        (route.wrapping_add(ROUTE_ACTIVE_BYTE_OFFSET) as *const u8).read()
    };
    if route_active != 0 {
        return 0;
    }

    let subtask = unsafe {
        (this.wrapping_add(CURRENT_SUBTASK_OFFSET) as *const u32).read_unaligned()
    };
    if subtask == 0 {
        return lf_checker_rt::callee_thiscall!(4, u32, this, event);
    }

    let subtask_state = lf_checker_rt::callee_thiscall!(1, u32, subtask);
    let controller = unsafe {
        (this.wrapping_add(CONTROLLER_OFFSET) as *const u32).read_unaligned()
    };
    let controller_state = lf_checker_rt::callee_thiscall!(2, u32, controller);
    if controller_state == subtask_state {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(3, u32, subtask)
});
