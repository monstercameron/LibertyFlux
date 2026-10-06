// original: 0x0092D2B0 scaled_square_times_global (proposed)

/// Multiply the square of the argument by a global factor, scaled by four.
///
/// Returns `((G + 8) * arg * arg) << 2` with 32-bit wrapping arithmetic,
/// where `G` is the dword at `GLOBAL`. All three multiplies are the
/// original's `imul` order: `arg * arg` first, then by `G + 8`, then the
/// shift.
///
/// Original: 0x0092D2B0 (cdecl, one stack word). Leaf: no calls, no writes.
lf_checker_rt::export!(cdecl, rw_0092D2B0(arg: u32) -> u32 {
    unsafe {
        const GLOBAL: u32 = 0x0163_37F0;
        let g = (lf_checker_rt::relocated(GLOBAL) as *const u32).read_unaligned();
        let sq = arg.wrapping_mul(arg);
        g.wrapping_add(8).wrapping_mul(sq).wrapping_shl(2)
    }
});
