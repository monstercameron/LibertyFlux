// original: 0x00B80AF0 ped_task_manager_request_bomb_task

/// Request the first applicable bomb task for this object. A nonzero word at
/// `+0x04` takes priority and calls the task interface with kind one; otherwise
/// a nonzero word at `+0x08` selects kind two. No call is made when both are
/// zero. The interface owns the three stack arguments; the wrapper's EAX is
/// not a meaningful result.
lf_checker_rt::export!(thiscall, rw_b80af0(this: u32) -> u32 {
    unsafe {
        const FIRST_TASK: u32 = 0x04;
        const SECOND_TASK: u32 = 0x08;
        const TASK_INTERFACE: u32 = 1;

        if ((this + FIRST_TASK) as *const u32).read_unaligned() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TASK_INTERFACE, u32, this, 0u32, 1u32, 0u32);
        } else if ((this + SECOND_TASK) as *const u32).read_unaligned() != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TASK_INTERFACE, u32, this, 0u32, 2u32, 0u32);
        }
        0
    }
});
