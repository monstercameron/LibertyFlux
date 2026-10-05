// original: 0x00a93930 stream_dispatch_request (proposed)

/// Dispatch a stream request key down the fast or the fallback path.
///
/// Probes the key with the first callee (key plus a global handle). A
/// non-zero low byte takes the fast path through the second callee and
/// returns its answer. Otherwise the third callee resolves the key: a null
/// answer is returned as is, else the fourth callee runs and its answer is
/// returned.
///
/// Original: cdecl, one stack argument. Four callees (cdecl, 2/2/1/1 args).
lf_checker_rt::export!(cdecl, rw_00a93930(key: u32) -> u32 {
    unsafe {
        const HANDLE_G: u32 = 0x0103e89c;
        const PROBE: u32 = 0;
        const FAST: u32 = 1;
        const RESOLVE: u32 = 2;
        const FALLBACK: u32 = 3;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let handle = rd32(lf_checker_rt::relocated(HANDLE_G));
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, key, handle);
        if probe & 0xff != 0 {
            return lf_checker_rt::callee_cdecl!(FAST, u32, key, handle);
        }
        let resolved: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, key);
        if resolved == 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(FALLBACK, u32, key)
    }
});
