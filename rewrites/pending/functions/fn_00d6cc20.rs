// original: 0x00d6cc20 replay_bar_scaled_position
/// Scale the progress ratio by the bar width and the selected time base.
///
/// Returns `(a / n) * w + o`, all in single precision, where `a` is the
/// argument, `n` the count at +0x100, `w`/`o` the floats at +0x10/+0x08, and
/// the final factor the global at 0x105C884 (or 0x105C888 when callee 1
/// answers nonzero). Integer inputs convert as unsigned.
lf_checker_rt::export!(thiscall, rw_00d6cc20(this_ptr: u32, a: u32) -> f64 {
    unsafe {
        use core::arch::x86::{
            _mm_add_ss, _mm_cvtss_f32, _mm_div_ss, _mm_mul_ss, _mm_set_ss,
        };
        let b = this_ptr as *const u8;
        let n = *((b.add(0x100)) as *const u32);
        let q = _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(a as f32), _mm_set_ss(n as f32)));
        let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let sel: u32 = if (ans & 0xFF) != 0 {
            *lf_checker_rt::global::<u32>(0x105c888)
        } else {
            *lf_checker_rt::global::<u32>(0x105c884)
        };
        let w = f32::from_bits(*((b.add(0x10)) as *const u32));
        let o = f32::from_bits(*((b.add(0x08)) as *const u32));
        let mut r = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(w), _mm_set_ss(q)));
        r = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(r), _mm_set_ss(o)));
        r = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(r), _mm_set_ss((sel as i32) as f32)));
        r as f64
    }
});
