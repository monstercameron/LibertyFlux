// original: 0x008FB1D0 stream_tables_reset
/// Reset every streaming table, pick rate constants, start up.
///
/// Clears the counters and mode byte, writes the idle pattern into
/// all 28 big-table entries (seven rows of four) and all 8
/// small-table entries, picks the rate float pair by the mode bytes
/// (default pair stays only when the mode differs from 0x6a while
/// the flags byte is zero), then initializes the global manager
/// object and tail-calls the manager startup. Cdecl, no arguments;
/// returns the tail call's result. Note: the batch size (233) ends
/// mid-function; the true size is 251 including the trailing call
/// and tail jump.
export!(cdecl, rw_008fb1d0() -> u32 {
    unsafe {
        const COUNT_B: u32 = 0x118e7d8;
        const COUNT_A: u32 = 0x118e7d4;
        const MODE: u32 = 0x118e7d0;
        const BIG: u32 = 0x118dec0;
        const SMALL: u32 = 0x118e5c0;
        const ENTRY: u32 = 0x40;
        const RATE_A: u32 = 0x1033120;
        const RATE_B: u32 = 0x1033124;
        #[inline(always)]
        unsafe fn idle(e: u32) {
            unsafe {
                ((e) as *mut u32).write_unaligned(0);
                ((e + 0x04) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e + 0x08) as *mut u32).write_unaligned(0);
                ((e + 0x0c) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e + 0x30) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e + 0x34) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e + 0x3b) as *mut u8).write(1);
            }
        }
        *global::<u32>(COUNT_B) = 0;
        *global::<u32>(COUNT_A) = 0;
        *global::<u8>(MODE) = 0;
        let big = relocated(BIG);
        for r in 0..7u32 {
            for k in 0..4u32 {
                idle(big.wrapping_add(
                    r.wrapping_mul(4).wrapping_add(k).wrapping_mul(ENTRY)));
            }
        }
        let small = relocated(SMALL);
        for k in 0..8u32 {
            idle(small.wrapping_add(k.wrapping_mul(ENTRY)));
        }
        *global::<u32>(RATE_A) = 0x3e8f5c29;
        *global::<u32>(RATE_B) = 0x3e8f5c29;
        let mode = *global::<u8>(0x116c250);
        if mode == 0x6a || *global::<u8>(0x116c253) != 0 {
            *global::<u32>(RATE_A) = 0x3ee66666;
            *global::<u32>(RATE_B) = 0x3eed9168;
        }
        callee_thiscall!(1, u32, relocated(0x1033130));
        callee_cdecl!(2, u32,)
    }
});
