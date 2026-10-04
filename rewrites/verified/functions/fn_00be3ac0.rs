// original: 0x00be3ac0 CTaskComplexWaitForBus::vf19 (symbols)

/// Build this task's wait-for-bus subtask, or null when the pool is empty.
///
/// Takes no inputs (both the object pointer and the stack word are unread).
/// Asks the task pool for a fresh slot and, when one is granted, constructs
/// the subtask in place with no further arguments. Returns the constructed
/// subtask, or zero when the pool had nothing to give.
///
/// Original: 0x00be3ac0 (thiscall, one stack word, both unread).
lf_checker_rt::export!(thiscall, rw_00be3ac0(_this: u32, _arg: u32) -> u32 {
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
