// original: 0x00be7c70 task_child_replace (proposed)

/// Replace the task's child, releasing the old one, and mark the flags.
///
/// `this` points to the task, `child` is the new child pointer. Releases the
/// old child at `+0x14` through its virtual slot 0 (callee 1, thiscall, one
/// stack word holding 1) when non-null and clears the slot, stores the new
/// child, then rewrites the flag word at `+0x28` clearing bit 4 and setting
/// bit 1. No meaningful return value (`ret: none`).
///
/// Original: thiscall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00be7c70(this: u32, child: u32) -> u32 {
    unsafe {
        const OFF_CHILD: u32 = 0x14;
        const OFF_FLAGS: u32 = 0x28;
        const CLEAR_BIT: u32 = 0x10;
        const SET_BIT: u32 = 0x02;

        let old = ((this + OFF_CHILD) as *const u32).read_unaligned();
        if old != 0 {
            let vtable = (old as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((vtable + 0) as *const u32).read_unaligned() as usize);
            release(old, 1);
            ((this + OFF_CHILD) as *mut u32).write_unaligned(0);
        }
        ((this + OFF_CHILD) as *mut u32).write_unaligned(child);
        let flags = ((this + OFF_FLAGS) as *const u32).read_unaligned();
        ((this + OFF_FLAGS) as *mut u32).write_unaligned((flags & !CLEAR_BIT) | SET_BIT);
        0
    }
});
