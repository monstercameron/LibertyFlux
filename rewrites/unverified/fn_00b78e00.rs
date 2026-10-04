// original: 0x00b78e00 channel_release_row (proposed)

/// Release a channel and run its row's teardown hooks.
///
/// Marks channel `ch` unused, resolves its row address, and passes it
/// through the two teardown callees, returning the second callee's answer.
/// A channel index of 64 or above jumps to a shared trap instead; the
/// contract keeps every trial below 64 so that branch is never taken.
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b78e00(ch: u32) -> u32 {
    unsafe {
        const USED: u32 = 0x0167CCE0;
        const ROW_TABLE: u32 = 0x0167CEA0;
        const ROW_STRIDE: u32 = 80;
        const CHANNELS: u32 = 64;
        const TEARDOWN_A: u32 = 1;
        const TEARDOWN_B: u32 = 2;
        if ch >= CHANNELS {
            return 0;
        }
        ((lf_checker_rt::relocated(USED) + ch) as *mut u8).write(0);
        let row = lf_checker_rt::relocated(ROW_TABLE) + ch.wrapping_mul(ROW_STRIDE);
        let mid: u32 = lf_checker_rt::callee_cdecl!(TEARDOWN_A, u32, row);
        lf_checker_rt::callee_thiscall!(TEARDOWN_B, u32, mid)
    }
});
