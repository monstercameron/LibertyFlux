// original: 0x00B7F180 CTaskComplexPlayerDrive::vf5

/// Attempts to abort the active child task and, when accepted or already
/// marked abortable, clears both active-task registry slots. A newly accepted
/// abort also sets child flag bit 1. The child pointer
/// is at `+0x08`, its flags at `+0x0c`, and virtual slot 5 is at vtable offset
/// `+0x14`. The receiver's registry records begin at `+0x14` and `+0x34`; the
/// reset sentinels are at `+0x20` and `+0x40`, with adjacent state bytes at
/// `+0x24` and `+0x44`. Three stack arguments are forwarded unchanged to the
/// child. A failed child response returns false before any registry call or
/// field reset; a successful path returns true in AL.
lf_checker_rt::export!(thiscall, rw_b7f180(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CHILD_TASK: u32 = 0x08;
        const TASK_FLAGS: u32 = 0x0c;
        const CHILD_ABORTABLE: u32 = 1;
        const CHILD_ABORT_ACCEPTED: u32 = 2;
        const FIRST_REGISTRY: u32 = 0x14;
        const FIRST_SENTINEL: u32 = 0x20;
        const FIRST_STATE: u32 = 0x24;
        const SECOND_REGISTRY: u32 = 0x34;
        const SECOND_SENTINEL: u32 = 0x40;
        const SECOND_STATE: u32 = 0x44;
        const INVALID_TASK: u32 = u32::MAX;
        const ABORTABLE_CALLEE: u32 = 1;
        const CLEAR_FIRST_REGISTRY: u32 = 2;
        const CLEAR_SECOND_REGISTRY: u32 = 3;

        let child = (this.wrapping_add(CHILD_TASK) as *const u32).read_unaligned();
        let flags = (child.wrapping_add(TASK_FLAGS) as *const u32).read_unaligned();
        if flags & CHILD_ABORTABLE == 0 {
            let can_abort = lf_checker_rt::callee_thiscall!(
                ABORTABLE_CALLEE,
                u32,
                child,
                arg0,
                arg1,
                arg2,
            ) as u8;
            if can_abort == 0 {
                return 0;
            }
            let flags_after_callback = (child.wrapping_add(TASK_FLAGS) as *const u32).read_unaligned();
            (child.wrapping_add(TASK_FLAGS) as *mut u32)
                .write_unaligned(flags_after_callback | CHILD_ABORT_ACCEPTED);
        }

        let first_registry = this.wrapping_add(FIRST_REGISTRY);
        let _ = lf_checker_rt::callee_thiscall!(
            CLEAR_FIRST_REGISTRY,
            u32,
            first_registry,
            INVALID_TASK,
        );
        (this.wrapping_add(FIRST_SENTINEL) as *mut u32).write_unaligned(INVALID_TASK);
        (this.wrapping_add(FIRST_STATE) as *mut u8).write(0);

        let second_registry = this.wrapping_add(SECOND_REGISTRY);
        let _ = lf_checker_rt::callee_thiscall!(
            CLEAR_SECOND_REGISTRY,
            u32,
            second_registry,
            INVALID_TASK,
        );
        (this.wrapping_add(SECOND_SENTINEL) as *mut u32).write_unaligned(INVALID_TASK);
        (this.wrapping_add(SECOND_STATE) as *mut u8).write(0);
        1
    }
});
