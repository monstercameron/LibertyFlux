// original: 0x00be0c00 CTaskSimpleWaitUntilAreaCodesMatch::vf1 (symbols)

/// Build this task's area-code-match waiter, or null when the pool is empty.
///
/// Reads only the area code at `this + AREA_OFF` (0x50). Asks the task pool
/// for a fresh slot and, when one is granted, constructs the subtask there
/// with that area code. Returns the constructed subtask, or zero when the
/// pool had nothing to give.
///
/// Original: 0x00be0c00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00be0c00(this: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const AREA_OFF: u32 = 0x50;
        const ALLOC: u32 = 1;
        const CONSTRUCT: u32 = 2;
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            return 0;
        }
        let area = (this.wrapping_add(AREA_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, slot, area)
    }
});
