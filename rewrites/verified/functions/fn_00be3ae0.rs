// original: 0x00be3ae0 CTaskComplexWaitForDryWeather::vf19 (symbols)

/// Build this task's wait-for-dry-weather subtask, or null when empty.
///
/// Takes no inputs (both the object pointer and the stack word are unread).
/// Asks the task pool for a fresh slot and, when one is granted, constructs
/// the subtask there with the fixed arguments (1000, 0, 0, 8.0f): the first
/// word is a millisecond-scale limit, the last the float 8.0 passed by bits.
/// Returns the constructed subtask, or zero when the pool had nothing.
///
/// Original: 0x00be3ae0 (thiscall, one stack word, both unread).
lf_checker_rt::export!(thiscall, rw_00be3ae0(_this: u32, _arg: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const LIMIT_MS: u32 = 1000;
        const WAIT_SECONDS_BITS: u32 = 0x41000000; // 8.0f
        const ALLOC: u32 = 1;
        const CONSTRUCT: u32 = 2;
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, slot, LIMIT_MS, 0, 0, WAIT_SECONDS_BITS)
    }
});
