// original: 0x00b356e0 forward_abcd_with_zero_gap_b
/// Forward `(a, b, c, d)` with a zero in the fourth slot.
lf_checker_rt::export!(cdecl, rw_b356e0(a: u32, b: u32, c: u32, d: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(2, u32, a, b, c, 0, d)
});
