// original: 0x00DA50F0 CTaskComplexShockingEventHurryAway::vf1

/// Clone this task: allocate a fresh object through the game memory
/// manager, copy-construct it from the embedded member at `+0x20` and
/// return the clone.
///
/// Allocation failure makes this return null.
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00da50f0(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        const MEMMGR_SLOT: u32 = 0x0167E2A0;
        const MEMBER_OFF: u32 = 0x20;

        let manager = (lf_checker_rt::relocated(MEMMGR_SLOT) as *const u32).read();
        let fresh: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if fresh == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, this + MEMBER_OFF)
    }
});
