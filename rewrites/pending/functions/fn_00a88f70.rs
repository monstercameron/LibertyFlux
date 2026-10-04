// original: 0x00a88f70 distance_scale_select
/// Pick a distance-based scale factor, bit-exact.
///
/// With mode (dword at `this + 0x146C`) between 0 and 9, measures the
/// distance between two 3-vectors, compares it against a threshold chosen
/// by the mode, and takes the threshold over distance (or a tiny constant
/// when below it, NaN included). With mode 10 or above and a matching
/// object argument, takes a fixed factor instead. Otherwise keeps the
/// stored factor. All arithmetic is single-precision SSE, bit-exact.
export!(thiscall, rw_00a88f70(this_obj: u32, arg0: u32, arg1: u32, arg2: u32, _arg3: u32) -> f32 {
    unsafe {
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss,
                              _mm_set_ss, _mm_sqrt_ss, _mm_sub_ss};
        let mode = *((this_obj.wrapping_add(0x146c)) as *const i32);
        let scale_ptr = *((this_obj.wrapping_add(0x1468)) as *const u32);
        let mut scale = *((scale_ptr.wrapping_add(0xc)) as *const f32);
        if mode >= 0 && mode <= 9 {
            let t = f32::from_bits(*global::<u32>(
                if mode > 3 { 0x103e848 } else { 0x103e84c }));
            let ax = *((arg1) as *const f32);
            let ay = *((arg1.wrapping_add(4)) as *const f32);
            let az = *((arg1.wrapping_add(8)) as *const f32);
            let bx = *((arg2) as *const f32);
            let by = *((arg2.wrapping_add(4)) as *const f32);
            let bz = *((arg2.wrapping_add(8)) as *const f32);
            let dx = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(ax), _mm_set_ss(bx)));
            let dy = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(ay), _mm_set_ss(by)));
            let dz = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(az), _mm_set_ss(bz)));
            let dx2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(dx), _mm_set_ss(dx)));
            let dy2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(dy), _mm_set_ss(dy)));
            let dz2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(dz), _mm_set_ss(dz)));
            let s01 = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(dx2), _mm_set_ss(dy2)));
            let s = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(s01), _mm_set_ss(dz2)));
            let d = _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(s)));
            // comiss + jb: taken when d < t or either is NaN.
            if d.is_nan() || t.is_nan() || d < t {
                scale = f32::from_bits(0x3a83126f);
            } else {
                scale = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(t), _mm_set_ss(d)));
            }
        }
        if mode >= 10
            && arg0 != 0
            && (*((arg0.wrapping_add(0x28)) as *const u32) & 0x3c0) == 0x80
            && *((arg0.wrapping_add(0x1300)) as *const u32) == 1
        {
            scale = f32::from_bits(*global::<u32>(0x103e850));
        }
        scale
    }
});
