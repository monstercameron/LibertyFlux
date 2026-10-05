// original: 0x00b4f250 get_field_1ch_if_6 (proposed)

/// Return the task's field at `+0x1C` when its kind query answers 6.
///
/// `this + 0x128` holds a task pointer (or null). When non-null the task's
/// virtual slot at `+0x04` is queried; unless it returns 6 the result is
/// null. On 6 the task pointer is re-read and the word at `+0x1C` is
/// returned.
///
/// Original: 0x00b4f250 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4f250(this: u32) -> u32 {
    unsafe {
        const TASK: u32 = 0x128;
        const KIND_SLOT: u32 = 0x04;
        const FIELD: u32 = 0x1c;
        const WANT_KIND: u32 = 6;
        let task = ((this + TASK) as *const u32).read_unaligned();
        if task == 0 {
            return 0;
        }
        let vt = (task as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + KIND_SLOT) as *const u32).read_unaligned() as usize);
        if query(task) != WANT_KIND {
            return 0;
        }
        let task2 = ((this + TASK) as *const u32).read_unaligned();
        ((task2 + FIELD) as *const u32).read_unaligned()
    }
});
