// original: 0x009FE1D0 frag_pair_update_guarded (proposed)

/// Run the two-sided frag update pair for `arg`, with guarded extras.
///
/// Calls the first update callee with (`arg`, 0), then, when the global
/// enable byte is nonzero, the first guarded callee with `arg`; then the
/// second update callee with (`arg`, 0) and, under the same guard, the
/// second guarded callee. Returns the last callee's answer.
///
/// Original: 0x009FE1D0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FE1D0(arg: u32) -> u32 {
    unsafe {
        const ENABLE: u32 = 0x01037651;
        lf_checker_rt::callee_cdecl!(1, u32, arg, 0);
        if (lf_checker_rt::relocated(ENABLE) as *const u8).read() != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, arg);
        }
        let r = lf_checker_rt::callee_cdecl!(3, u32, arg, 0);
        if (lf_checker_rt::relocated(ENABLE) as *const u8).read() != 0 {
            lf_checker_rt::callee_cdecl!(4, u32, arg)
        } else {
            r
        }
    }
});
