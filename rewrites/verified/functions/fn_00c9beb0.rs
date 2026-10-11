// original: 0x00C9BEB0 GetTaskContextOrZero

/// Ask the lookup helper for the object associated with the stack key. Return zero for a null result; otherwise return the dword at object offset +0x14. The function removes its one incoming stack word.
lf_checker_rt::export!(stdcall, rw_00c9beb0(task_key: u32) -> u32 {
    const LOOKUP: u32 = 1;
    const CONTEXT_VALUE: u32 = 0x14;
    let object = lf_checker_rt::callee_stdcall!(LOOKUP, u32, task_key);
    if object == 0 {
        0
    } else {
        unsafe { (object.wrapping_add(CONTEXT_VALUE) as *const u32).read_unaligned() }
    }
});
