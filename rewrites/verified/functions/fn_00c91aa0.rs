// original: 0x00C91AA0 FindTaskWithMatchingFieldAndApplyValue

/// Look up the task using selector zero. A null lookup returns zero. For a non-null task, compare its dword at +0x64 with the second stack argument; return the task pointer if they differ, or call the task's value setter with the first stack argument and return its EAX value when they match. This stdcall wrapper removes both arguments.
lf_checker_rt::export!(stdcall, rw_00c91aa0(value: u32, expected_field: u32) -> u32 {
    const LOOKUP: u32 = 1;
    const SET_VALUE: u32 = 2;
    const MATCHED_FIELD: u32 = 0x64;
    let task = lf_checker_rt::callee_stdcall!(LOOKUP, u32, 0u32);
    if task == 0 {
        task
    } else {
        let field = unsafe { (task.wrapping_add(MATCHED_FIELD) as *const u32).read_unaligned() };
        if field == expected_field {
            lf_checker_rt::callee_thiscall!(SET_VALUE, u32, task, value)
        } else {
            task
        }
    }
});
