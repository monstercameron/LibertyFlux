// original: 0x00C91A90 FindTaskAndApplyZero

/// Forward the task key through the manager wrapper with a zero selector. The original pushes zero as the first stack argument and the incoming key as the second, calls the manager, returns its EAX value, and removes the incoming four-byte argument.
lf_checker_rt::export!(stdcall, rw_00c91a90(task_key: u32) -> u32 {
    const MANAGER_LOOKUP: u32 = 1;
    lf_checker_rt::callee_stdcall!(MANAGER_LOOKUP, u32, 0u32, task_key)
});
