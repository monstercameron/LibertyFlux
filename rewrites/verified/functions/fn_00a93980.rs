// original: 0x00a93980 stream_query_device (proposed)

/// Query a stream device, forwarding to the three-argument callee on hit.
///
/// Probes `(key, handle)` with the first callee. A zero low byte returns
/// the probe answer unchanged; otherwise the second callee runs with
/// `(key, handle, sub)` and its answer is returned.
///
/// Original: cdecl, two stack arguments. Two callees (cdecl, 2/3 args).
lf_checker_rt::export!(cdecl, rw_00a93980(key: u32, sub: u32) -> u32 {
    unsafe {
        const HANDLE_G: u32 = 0x0103e89c;
        const PROBE: u32 = 0;
        const QUERY: u32 = 1;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let handle = rd32(lf_checker_rt::relocated(HANDLE_G));
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, key, handle);
        if probe & 0xff == 0 {
            return probe;
        }
        lf_checker_rt::callee_cdecl!(QUERY, u32, key, handle, sub)
    }
});
