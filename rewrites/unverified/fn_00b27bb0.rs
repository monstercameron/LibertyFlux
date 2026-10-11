// original: 0x00B27BB0 find_first_available_task_id

/// Query task IDs from 0x37 through 0x3c in order, returning the first
/// nonzero result and stopping immediately. If every lookup returns zero,
/// return zero. The function has a callee-cleaned one-word stack argument;
/// each helper call is thiscall with that pointer in ECX and the task ID on
/// the stack.
lf_checker_rt::export!(stdcall, rw_b27bb0(owner: u32) -> u32 {
    unsafe {
        const FIRST_TASK_ID: u32 = 0x37;
        const LAST_TASK_ID: u32 = 0x3c;
        let mut task_id = FIRST_TASK_ID;
        while task_id <= LAST_TASK_ID {
            let found = lf_checker_rt::callee_thiscall!(1, u32, owner, task_id);
            if found != 0 {
                return found;
            }
            task_id += 1;
        }
        0
    }
});
