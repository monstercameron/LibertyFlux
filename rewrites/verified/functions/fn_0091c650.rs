// original: 0x0091C650 text_max_char_metric
/// Reduce a per-position metric over a wide string to its maximum.
///
/// Returns `0.0` for a null or empty string. Otherwise walks the string: at
/// each position the metric helper yields a float that replaces the running
/// maximum when strictly greater (so `NaN` results are ignored, exactly like
/// the original's `comiss`/`jbe`), and the advance helper yields the next
/// position. The walk ends at a NUL word, and the running maximum (kept in
/// the incoming arg0 stack slot by the original, in a local here) is
/// returned on `ST0`.
export!(cdecl, rw_0091c650(a0: u32) -> f32 {
    unsafe {
        if a0 == 0 || *(a0 as *const u16) == 0 {
            return 0.0;
        }
        let mut m: f32 = 0.0;
        let mut esi = a0;
        loop {
            let r: f32 = callee_cdecl!(1, f32, esi, 0);
            if r > m {
                m = r;
            }
            esi = callee_cdecl!(2, u32, esi);
            if *(esi as *const u16) == 0 {
                break;
            }
            esi = esi.wrapping_add(2);
            if *(esi as *const u16) == 0 {
                break;
            }
        }
        m
    }
});
