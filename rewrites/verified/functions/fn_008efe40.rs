// original: 0x008efe40 scaled_callee_to_int
/// Scale the two-argument helper's float answer, return an integer.
///
/// Forwards both arguments to the helper (direct call, stubbed by the
/// checker) and converts its float answer times a constant factor to an
/// integer with truncation (out-of-range and NaN yield 0x80000000).
export!(cdecl, rw_008efe40(a0: u32, a1: u32) -> u32 {
    unsafe {
        let r = callee_cdecl!(1, f32, a0, a1);
        let z = r * *global::<f32>(0xFE8BC8);
        let t = z.trunc();
        if t.is_nan() || t < -2147483648.0 || t >= 2147483648.0 {
            0x80000000u32
        } else {
            (t as i32) as u32
        }
    }
});
