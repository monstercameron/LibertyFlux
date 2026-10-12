// original: 0x00C0B560 cutscene_end_timer_or_max (proposed)

/// Cutscene end timer, or a large default when no cutscene finished.
///
/// Takes no arguments. Calls the cutscene-finished query (intercepted
/// callee 1, no arguments); only its low byte is read. When that byte is
/// zero, or when the global timer word at `TIMER` is negative as a
/// SIGNED 32-bit value, returns `DEFAULT` (9999). Otherwise returns the
/// timer word itself. The signedness matters: a large unsigned timer
/// with the top bit set still yields the default.
///
/// Original: 0x00C0B560 (cdecl, no arguments; query at 0x009C37D0,
/// timer global at 0x01294758).
lf_checker_rt::export!(cdecl, rw_00c0b560() -> u32 {
    unsafe {
        /// Recorder id of the cutscene-finished query call.
        const QUERY: u32 = 1;
        /// Global word holding the cutscene end timer.
        const TIMER: u32 = 0x0129_4758;
        /// Default returned when no finished cutscene left a timer.
        const DEFAULT: u32 = 9999;

        let done: u32 = lf_checker_rt::callee_cdecl!(QUERY, u32);
        if done & 0xFF == 0 {
            return DEFAULT;
        }
        let timer = (lf_checker_rt::global::<u32>(TIMER) as *const u32).read();
        if (timer as i32) < 0 {
            DEFAULT
        } else {
            timer
        }
    }
});
