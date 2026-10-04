// original: 0x008efa80 double_scaled_call_to_int
/// Scale the helper's float answer in double precision, return an integer.
///
/// Calls the nullary helper (direct call, stubbed by the checker; the real
/// one reads entry ECX, which the stubbed model does not observe), widens
/// its answer to double precision, multiplies by a constant, and converts
/// to an integer with truncation (out-of-range and NaN yield 0x80000000).
export!(cdecl, rw_008efa80() -> u32 {
    unsafe {
        let v = callee_cdecl!(1, f32,);
        let d = (v as f64) * *global::<f64>(0xE837F8);
        let t = d.trunc();
        if t.is_nan() || t < -2147483648.0 || t >= 2147483648.0 {
            0x80000000u32
        } else {
            (t as i32) as u32
        }
    }
});
