// original: 0x00be0be0 CTaskSimpleWaitForBus::vf1 (symbols)

/// Build this task's wait-for-bus subtask, or null when the pool is empty.
///
/// Takes no inputs (the object pointer is unread and there are no stack
/// words). Asks the task pool for a fresh slot: on failure returns zero,
/// otherwise tail-jumps to the subtask constructor with the slot. The rewrite
/// expresses the tail jump as a plain call returning the constructor's
/// answer; the observable calls and result are identical.
///
/// Original: 0x00be0be0 (thiscall, no stack arguments; tail jump).
lf_checker_rt::export!(thiscall, rw_00be0be0(_this: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const ALLOC: u32 = 1;
        const CONSTRUCT: u32 = 2;
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, slot)
    }
});
