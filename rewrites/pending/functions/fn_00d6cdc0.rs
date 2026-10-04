// original: 0x00d6cdc0 replay_bar_clamped_index
/// Map a position to a clamped, scaled slot index.
///
/// Clamps `pos` into [`lo`, `hi`] (fields +0xEC/+0xD0, NaN keeps the input),
/// optionally clamps again into [+0xB0, +0xBC] when `flag` is nonzero, then
/// returns `round_half_away((pos - lo) * n / (hi - lo))` where `n` is the
/// unsigned count at +0x100. Out-of-range results yield `i32::MIN`, matching
/// the original's truncate conversion.
lf_checker_rt::export!(thiscall, rw_00d6cdc0(this_ptr: u32, pos_bits: u32, flag: u32) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss, _mm_sub_ss,
        };
        let b = this_ptr as *const u8;
        let n = *((b.add(0x100)) as *const u32);
        let hi = f32::from_bits(*((b.add(0xd0)) as *const u32));
        let lo = f32::from_bits(*((b.add(0xec)) as *const u32));
        let mut v = f32::from_bits(pos_bits);
        let span = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(hi), _mm_set_ss(lo)));
        let ratio = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(n as f32), _mm_set_ss(span)));
        if lo > v {
            v = lo;
        }
        if v > hi {
            v = hi;
        }
        if (flag & 0xFF) != 0 {
            let lo2 = f32::from_bits(*((b.add(0xb0)) as *const u32));
            let hi2 = f32::from_bits(*((b.add(0xbc)) as *const u32));
            if lo2 > v {
                v = lo2;
            }
            if v > hi2 {
                v = hi2;
            }
        }
        v = _mm_cvtss_f32(_mm_sub_ss(_mm_set_ss(v), _mm_set_ss(lo)));
        v = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(v), _mm_set_ss(ratio)));
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
