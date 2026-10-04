// original: 0x00d6ce70 replay_bar_scaled_index
/// Map a position to a scaled slot index without clamping.
///
/// Returns `round_half_away((pos - lo) * n / (hi - lo))` with `lo`/`hi` from
/// +0xEC/+0xD0 and unsigned `n` from +0x100. Out-of-range results yield
/// `i32::MIN`, matching the original's truncate conversion.
lf_checker_rt::export!(thiscall, rw_00d6ce70(this_ptr: u32, pos_bits: u32) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss, _mm_sub_ss,
        };
        let b = this_ptr as *const u8;
        let n = *((b.add(0x100)) as *const u32);
        let hi = f32::from_bits(*((b.add(0xd0)) as *const u32));
        let lo = f32::from_bits(*((b.add(0xec)) as *const u32));
        let pos = f32::from_bits(pos_bits);
        let span = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(hi), _mm_set_ss(lo)));
        let mut v = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(n as f32), _mm_set_ss(span)));
        let off = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(pos), _mm_set_ss(lo)));
        v = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(v), _mm_set_ss(off)));
        let r = if 0.0f32 > v {
            _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(v), _mm_set_ss(0.5)))
        } else {
            _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(v), _mm_set_ss(0.5)))
        };
        if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
            i32::MIN as u32
        } else {
            (r as i32) as u32
        }
    }
});
