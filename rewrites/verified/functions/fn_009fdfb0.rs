// original: 0x009FDFB0 frag_pools_init (proposed)

/// Initialise the two global frag pools, then tail into the finaliser.
///
/// Runs the shared pool-initialiser callee with 1000 on the first global
/// pool object and with 4500 on the second, then tail-calls the finaliser
/// callee with no stack arguments. `ecx` at the tail jump is whatever the
/// initialiser left behind (the original sets nothing between the second
/// call and the jump), so it is left unset here too and uncompared.
/// Returns the finaliser's answer.
///
/// Original: 0x009FDFB0 (cdecl, no arguments; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_009FDFB0() -> u32 {
    unsafe {
        const POOL_A: u32 = 0x012BCC94;
        const POOL_B: u32 = 0x012BCCD0;
        const SIZE_A: u32 = 1000;
        const SIZE_B: u32 = 4500;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(POOL_A), SIZE_A);
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(POOL_B), SIZE_B);
        lf_checker_rt::callee_cdecl!(2, u32,)
    }
});
