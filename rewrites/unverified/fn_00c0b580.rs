// original: 0x00C0B580 cutscene_scaled_timer_or_max (proposed)

/// Cutscene timer scaled by two thirds, or a large default.
///
/// Takes no arguments. Calls the cutscene-finished query (intercepted
/// callee 1, no arguments); only its low byte is read. When that byte is
/// zero, or when the global timer word at `TIMER` is negative as a
/// SIGNED 32-bit value, returns `DEFAULT` (9999). Otherwise returns
/// `(2 * timer) / 3`, computed exactly as the original: the doubling
/// wraps modulo 2^32, then a signed 64-bit multiply by the magic
/// constant `MAGIC` (0x55555556) keeps the high word, and a sign fixup
/// (`(hi >> 31) + hi`, wrapping) rounds toward zero for the wrapped
/// negative doublings of large timers. `timer` itself is non-negative
/// here, but the doubled value may carry the sign bit.
///
/// Original: 0x00C0B580 (cdecl, no arguments; query at 0x009C37D0,
/// timer global at 0x01294754).
lf_checker_rt::export!(cdecl, rw_00c0b580() -> u32 {
    unsafe {
        /// Recorder id of the cutscene-finished query call.
        const QUERY: u32 = 1;
        /// Global word holding the cutscene timer.
        const TIMER: u32 = 0x0129_4754;
        /// Default returned when no finished cutscene left a timer.
        const DEFAULT: u32 = 9999;
        /// Magic multiplier for the signed division by three.
        const MAGIC: i64 = 0x5555_5556;

        let done: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32);
        if done & 0xFF == 0 {
            return DEFAULT;
        }
        let timer = (lf_checker_rt::global::<u32>(TIMER) as *const u32).read();
        if (timer as i32) < 0 {
            return DEFAULT;
        }
        let doubled = timer.wrapping_mul(2);
        let prod = (doubled as i32 as i64).wrapping_mul(MAGIC);
        let hi = (prod >> 32) as i32;
        ((hi as u32 >> 31) as i32).wrapping_add(hi) as u32
    }
});
