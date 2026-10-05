// original: 0x00c849a0 scenario_stage_chain (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `frame_pointer_args` (plus encrypted bytes): four
// callees take pointers to the function's own aligned stack frame. Kept for a
// future checker or re-run; never passed, not verified.

/// Run the four scenario staging passes over chained scratch buffers.
///
/// Each pass takes two scratch pointers into the frame plus (for the first and
/// third) a zero flag word, chaining the second pointer of one pass into the
/// first of the next. A standard security cookie guards the frame and is
/// checked on exit. Takes no arguments and returns the cookie check's result.
///
/// Original: cdecl, no stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c849a0() -> u32 {
    unsafe {
        const COOKIE: u32 = 0x1057fb4;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        const C_CHECK: u32 = 5;
        let mut b0 = [0u32; 4];
        let mut b1 = [0u32; 4];
        let mut b2 = [0u32; 4];
        let mut b3 = [0u32; 4];
        let mut b4 = [0u32; 4];
        b0[3] = 0;
        lf_checker_rt::callee_cdecl!(C1, u32, b1.as_mut_ptr() as u32, b0.as_mut_ptr() as u32);
        lf_checker_rt::callee_cdecl!(C2, u32, b2.as_mut_ptr() as u32, b1.as_mut_ptr() as u32);
        b2[3] = 0;
        lf_checker_rt::callee_cdecl!(C3, u32, b3.as_mut_ptr() as u32, b2.as_mut_ptr() as u32);
        lf_checker_rt::callee_cdecl!(C4, u32, b4.as_mut_ptr() as u32, b3.as_mut_ptr() as u32);
        let ck = lf_checker_rt::global::<u32>(COOKIE).read_unaligned();
        lf_checker_rt::callee_thiscall!(C_CHECK, u32, ck)
    }
});
