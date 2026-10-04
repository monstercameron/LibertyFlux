// original: 0x00b356a0 forward_abc_with_two_zeros
/// Forward `(a, b, c)` to the five-argument worker with two trailing zeros.
lf_checker_rt::export!(cdecl, rw_b356a0(a: u32, b: u32, c: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(2, u32, a, b, c, 0, 0)
});
