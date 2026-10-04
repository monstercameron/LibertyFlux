// original: 0x00be4170 CTaskComplexSitIdle::vf18 (symbols)

/// Build this task's sit-idle subtask, or null when the pool is empty.
///
/// Takes no inputs (both the object pointer and the stack word are unread).
/// Asks the task pool for a fresh slot and, when one is granted, constructs
/// the subtask there with the fixed arguments (0, -1.0f, 0, 0, 0, -1): the
/// second word is the float -1.0 passed by bits, the last a -1 selector.
/// Returns the constructed subtask, or zero when the pool had nothing.
///
/// Original: 0x00be4170 (thiscall, one stack word, both unread).
lf_checker_rt::export!(thiscall, rw_00be4170(_this: u32, _arg: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const NEG_ONE_BITS: u32 = 0xbf800000; // -1.0f
        const NO_CHOICE: u32 = 0xffffffff;
        const ALLOC: u32 = 1;
        const CONSTRUCT: u32 = 2;
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, slot, 0, NEG_ONE_BITS, 0, 0, 0, NO_CHOICE)
    }
});
