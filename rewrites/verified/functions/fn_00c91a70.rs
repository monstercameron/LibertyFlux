// original: 0x00C91A70 FindTaskAndApplyValue

/// Look up the task object using the first stack argument. If the lookup returns null, leave the value unchanged and return the lookup result. Otherwise call the object's value setter with the second stack argument and return that call's EAX value. This stdcall wrapper removes both stack arguments.
lf_checker_rt::export!(stdcall, rw_00c91a70(lookup_key: u32, value: u32) -> u32 {
    const LOOKUP: u32 = 1;
    const SET_VALUE: u32 = 2;
    let task = lf_checker_rt::callee_stdcall!(LOOKUP, u32, lookup_key);
    if task == 0 {
        task
    } else {
        lf_checker_rt::callee_thiscall!(SET_VALUE, u32, task, value)
    }
});
