// original: 0x00D39D20 CDummyTask_Dead::vf8

/// Write status value 2 to the output word, query the task manager, and, when it returns an object, forward the task's +0x1c value with arguments 0 and 1. Return that helper's result, or zero when the manager has no object. ECX is `this`; the callee removes two stack arguments.
lf_checker_rt::export!(thiscall, rw_00d39d20(this: u32, _first: u32, output: u32) -> u32 {
    unsafe {
        const STATUS_WORD: u32 = 2;
        const TASK_MEMBER: u32 = 0x1c;
        const MANAGER_GLOBAL: u32 = 0x0167e2a0;
        const GET_MANAGER_OBJECT: u32 = 1;
        const RUN_STATUS_QUERY: u32 = 2;
        (output as *mut u32).write_unaligned(STATUS_WORD);
        let manager = lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned();
        let object = lf_checker_rt::callee_thiscall!(GET_MANAGER_OBJECT, u32, manager);
        if object == 0 {
            0
        } else {
            let member = (this.wrapping_add(TASK_MEMBER) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(RUN_STATUS_QUERY, u32, object, member, 0, 1)
        }
    }
});
