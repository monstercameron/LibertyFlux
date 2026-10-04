// original: 0x009FE020 frag_pools_drain (proposed)

/// Drain both global frag pools through the shared drain callee.
///
/// Calls the drain callee on the first global pool object, then tail-calls
/// it on the second (`ecx` holds the second pool, observed in the call
/// log). Returns the second call's answer.
///
/// Original: 0x009FE020 (cdecl, no arguments; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_009FE020() -> u32 {
    unsafe {
        const POOL_A: u32 = 0x012BCC94;
        const POOL_B: u32 = 0x012BCCD0;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(POOL_A));
        lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(POOL_B))
    }
});
