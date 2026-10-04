// original: 0x00d2a720 range_check_sub3 (proposed)
/// True (1) when `v - 3 <= 0xb`, i.e. v in [3, 14].
///
/// The original computes `t = v - 3`, compares against 0xb and sets only the
/// low byte (`setbe al`), so the upper 24 bits of the result are the upper
/// bits of `t`, not zero. Cdecl, one stack word, returns full eax.
lf_checker_rt::export!(cdecl, rw_00d2a720(v: u32) -> u32 {
    let t = v.wrapping_sub(3);
    (t & 0xffff_ff00) | u32::from(t <= 0x0b)
});
