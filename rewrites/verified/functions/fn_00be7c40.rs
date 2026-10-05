// original: 0x00be7c40 task_slot_replace_guarded (proposed)

/// Replace the task's guarded child slot unless it is locked.
///
/// `this` points to the task, `child` is the new child pointer. If bit 2 of
/// the flag word at `+0x28` is set, does nothing. Otherwise releases the old
/// child at `+0x18` through its virtual slot 0 (callee 1, thiscall, one stack
/// word holding 1) when non-null, clears the slot, sets bit 0 of the flags
/// and stores the new child. No meaningful return value (`ret: none`).
///
/// Original: thiscall, one stack word, the callee pops 4 bytes.
lf_checker_rt::export!(thiscall, rw_00be7c40(this: u32, child: u32) -> u32 {
    unsafe {
        const OFF_CHILD: u32 = 0x18;
        const OFF_FLAGS: u32 = 0x28;
        const LOCK_BIT: u32 = 4;
        const SET_BIT: u32 = 1;

        let flags = ((this + OFF_FLAGS) as *const u32).read_unaligned();
        if flags & LOCK_BIT != 0 {
            return 0;
        }
        let old = ((this + OFF_CHILD) as *const u32).read_unaligned();
        if old != 0 {
            let vtable = (old as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((vtable + 0) as *const u32).read_unaligned() as usize);
            release(old, 1);
            ((this + OFF_CHILD) as *mut u32).write_unaligned(0);
        }
        ((this + OFF_FLAGS) as *mut u32).write_unaligned(flags | SET_BIT);
        ((this + OFF_CHILD) as *mut u32).write_unaligned(child);
        0
    }
});
