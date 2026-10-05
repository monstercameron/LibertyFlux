// original: 0x00c80ac0 scenario_arg_rewrite (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `other`: the listed range (0xC80AC0, 154 bytes) is
// a mis-split. Its first 21 bytes form a wrapper (rewrite the stack argument
// through `c1`, then jump to 0xC80A30, outside the range); the rest belongs to
// the following function, which also holds an unrelocated absolute read of
// writable game data that the checker cannot serve here. Only the wrapper is
// rewritten below; the range itself cannot be verified as listed.

/// Rewrite the argument through `c1`, then continue at the scenario core.
///
/// Pushes `arg` for `c1`, stores the result back over it, and jumps to the
/// core routine (modelled here as a call, since Rust cannot tail-jump).
///
/// Original: wrapper head is cdecl-shaped, one stack word.
lf_checker_rt::export!(cdecl, rw_00c80ac0(arg: u32) -> u32 {
    unsafe {
        const C1: u32 = 1;
        const C_CORE: u32 = 2;
        let rewritten: u32 = lf_checker_rt::callee_cdecl!(C1, u32, arg);
        lf_checker_rt::callee_cdecl!(C_CORE, u32, rewritten)
    }
});
