// original: 0x00b3a210 task_or_reposition (proposed)

/// When the task object at `obj` has a live current task (the task pointer
/// at its task slot is set and the task's active flag is nonzero), do
/// nothing and return the task address with its low byte cleared. Otherwise
/// forward `obj`, the `key` word, the `scale` word and the `mode` word to the
/// reposition callee (key and scale swap order across the call) and return
/// its result. Original: 0x00b3a210 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00b3a210(obj: u32, key: u32, scale: u32, mode: u32) -> u32 {
    unsafe {
        const TASK_SLOT: u32 = 0x6c;
        const ACTIVE_FLAG: u32 = 0x0e;
        const REPOSITION: u32 = 1;
        let task = ((obj + TASK_SLOT) as *const u32).read();
        if task != 0 && ((task + ACTIVE_FLAG) as *const u8).read() != 0 {
            return task & 0xffff_ff00;
        }
        lf_checker_rt::callee_cdecl!(REPOSITION, u32, obj, scale, key, mode)
    }
});
