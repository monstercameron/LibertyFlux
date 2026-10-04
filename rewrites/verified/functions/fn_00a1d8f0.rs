// original: 0x00a1d8f0 cam_forward_pair_zero (proposed)

/// Forwards one argument plus a zero word to a worker callee.
///
/// `a0` is passed through as the callee's first argument with a constant
/// zero second argument; the callee cleans nothing (cdecl) and its return
/// value is the function's own return value.
///
/// Original: 0x00a1d8f0 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a1d8f0(a0: u32) -> u32 {
    unsafe {
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(CALLEE, u32, a0, 0)
    }
});
