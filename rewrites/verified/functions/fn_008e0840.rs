// original: 0x008e0840 sweep_and_tail (proposed)

/// Run an unflagged sweep, then tail-jump into the flush routine.
///
/// Calls the sweep (callee 1) with flag 0 and tail-jumps to the flush
/// (callee 2, no arguments), whose answer is the return value. The rewrite
/// performs the tail as a call that forwards the result. Cdecl, no stack
/// arguments (the listed size covers two neighbouring entries; the real
/// body is the 10 bytes ending in the jump).
lf_checker_rt::export!(cdecl, rw_008e0840() -> u32 {
    unsafe {
        const CALLEE_SWEEP: u32 = 1;
        const CALLEE_FLUSH: u32 = 2;
        lf_checker_rt::callee_cdecl!(CALLEE_SWEEP, u32, 0);
        lf_checker_rt::callee_cdecl!(CALLEE_FLUSH, u32,)
    }
});
