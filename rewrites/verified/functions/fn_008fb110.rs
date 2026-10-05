// original: 0x008FB110 stream_pool_init_20
/// Size both streaming pools to 20 entries and pick rate constants.
///
/// Calls the pool allocator with 20 on each global pool, writes the
/// default float pair, then overwrites it with the alternate pair
/// unless the mode byte differs from 0x6a while the flags byte is
/// zero (in which case the default pair stays).
/// Cdecl, no arguments; returns the second call's result.
export!(cdecl, rw_008fb110() -> u32 {
    unsafe {
        const POOL_A: u32 = 0x118e7c0;
        const POOL_B: u32 = 0x118e7c8;
        const RATE_A: u32 = 0x1033120;
        const RATE_B: u32 = 0x1033124;
        const MODE: u32 = 0x116c250;
        const FLAGS: u32 = 0x116c253;
        const SIZE: u32 = 0x14;
        callee_thiscall!(1, u32, relocated(POOL_A), SIZE);
        let ans: u32 = callee_thiscall!(2, u32, relocated(POOL_B), SIZE);
        *global::<u32>(RATE_A) = 0x3e8f5c29;
        *global::<u32>(RATE_B) = 0x3e8f5c29;
        let mode = *global::<u8>(MODE);
        if mode == 0x6a || *global::<u8>(FLAGS) != 0 {
            *global::<u32>(RATE_A) = 0x3ee66666;
            *global::<u32>(RATE_B) = 0x3eed9168;
        }
        ans
    }
});
