// original: 0x00DA5120 CTaskComplexShockingEventWatch::vf1

/// Clone this task: allocate a fresh object through the game memory
/// manager, copy-construct it from the embedded member at `+0x20` and
/// return the clone, then copy the state word at `+0x70` from the old object.
///
/// Allocation failure still stores through the null pointer and faults, exactly like the original.
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00da5120(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const MEMMGR_SLOT: u32 = 0x0167E2A0;
        const MEMBER_OFF: u32 = 0x20;

        let manager = (lf_checker_rt::relocated(MEMMGR_SLOT) as *const u32).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        let clone: u32 = if fresh == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, this + MEMBER_OFF)
        };
        let v = ((this + 0x70) as *const u32).read();
        ((clone + 0x70) as *mut u32).write(v);
        clone
    }
});
