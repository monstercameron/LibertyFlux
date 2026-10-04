// original: 0x00d6f250 replay_bar_millis_rounded
/// Convert the seconds field at +0x04 to truncated milliseconds.
///
/// Multiplies the float at +0x04 by 1000 and truncates toward zero.
/// Unrepresentable results (NaN or beyond int64 range) yield 0, matching the
/// original's truncate-to-qword conversion whose low dword is returned.
lf_checker_rt::export!(thiscall, rw_00d6f250(this_ptr: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let b = this_ptr as *const u8;
        let secs = f32::from_bits(*((b.add(4)) as *const u32));
        let v = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(secs), _mm_set_ss(1000.0)));
        if v.is_nan() || v >= 9223372036854775808.0 || v <= -9223372036854775808.0 {
            0
        } else {
            (v as i64) as u32
        }
    }
});
