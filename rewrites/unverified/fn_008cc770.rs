// original: 0x008CC770 stream_flush_loop (proposed)

/// Flushes the streaming queue: calls the status callee (callee 0) and
/// returns at once when its low byte is 0. Otherwise repeats: call the pump
/// callee (callee 1) with 0, call the commit callee (callee 2) with 0x10,
/// then call the status callee again, until the status low byte is 0.
///
/// No arguments (cdecl); no return value. Every iteration makes all three
/// calls in order; the contract's answer sequence bounds the loop.
lf_checker_rt::export!(cdecl, rw_008CC770() -> u32 {
    unsafe {
        /// Status callee id.
        const STATUS: u32 = 0;
        /// Pump callee id.
        const PUMP: u32 = 1;
        /// Commit callee id.
        const COMMIT: u32 = 2;
        /// Argument passed to the commit callee.
        const COMMIT_ARG: u32 = 0x10;
        let first: u32 = lf_checker_rt::callee_cdecl!(STATUS, u32);
        if first & 0xFF == 0 {
            return 0;
        }
        loop {
            let _p: u32 = lf_checker_rt::callee_cdecl!(PUMP, u32, 0);
            let _c: u32 = lf_checker_rt::callee_cdecl!(COMMIT, u32, COMMIT_ARG);
            let s: u32 = lf_checker_rt::callee_cdecl!(STATUS, u32);
            if s & 0xFF == 0 {
                break;
            }
        }
        0
    }
});
