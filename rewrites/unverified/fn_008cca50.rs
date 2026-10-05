// original: 0x008CCA50 stream_drain_step (proposed)

/// Runs one streaming drain step: returns at once when the global request
/// byte is 0, otherwise clears it. When the global mode byte is non-zero,
/// calls the drain callee (callee 0) with argument 2 and returns. Otherwise
/// calls the reset callee (callee 1) with argument 0, then calls the drain
/// callee with argument 0 and clears the mode byte, but only when the global
/// state word is 0x46 or 0x32.
///
/// No arguments (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CCA50() -> u32 {
    unsafe {
        /// Global drain-request byte, cleared when seen.
        const REQUEST: u32 = 0x1172FDF;
        /// Global mode byte selecting the drain path.
        const MODE: u32 = 0x11609F6;
        /// Global state word gating the second drain call.
        const STATE: u32 = 0x1160C24;
        /// State values that allow the second drain call.
        const STATE_A: u32 = 0x46;
        const STATE_B: u32 = 0x32;
        /// Drain callee id.
        const DRAIN: u32 = 0;
        /// Reset callee id.
        const RESET: u32 = 1;
        if (lf_checker_rt::relocated(REQUEST) as *const u8).read() == 0 {
            return 0;
        }
        (lf_checker_rt::relocated(REQUEST) as *mut u8).write(0);
        if (lf_checker_rt::relocated(MODE) as *const u8).read() != 0 {
            let _d: u32 = lf_checker_rt::callee_cdecl!(DRAIN, u32, 2);
            return 0;
        }
        let _r: u32 = lf_checker_rt::callee_cdecl!(RESET, u32, 0);
        let state = (lf_checker_rt::relocated(STATE) as *const u32).read();
        if state != STATE_A && state != STATE_B {
            return 0;
        }
        let _d: u32 = lf_checker_rt::callee_cdecl!(DRAIN, u32, 0);
        (lf_checker_rt::relocated(MODE) as *mut u8).write(0);
        0
    }
});
