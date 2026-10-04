// original: 0x00a88ee0 range_gate_update
/// Gate a range state on the length of a callee-supplied vector.
///
/// Calls vtable slot 0x3B of the argument object (intercepted,
/// thiscall/1) to obtain a 3-vector, measures its length in
/// single-precision SSE, and compares it against the fixed threshold.
/// Above the threshold it sets flag bits 0-1 and stores 1.0; at or below
/// (NaN included) it sets bits 0, 1 and 3 and stores the small constant.
/// Returns the callee answer with its low byte replaced on the long path,
/// exactly as the original leaves EAX.
export!(thiscall, rw_00a88ee0(this_obj: u32, arg0: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_add_ss, _mm_cvtss_f32, _mm_mul_ss, _mm_set_ss,
                              _mm_sqrt_ss};
        *((this_obj.wrapping_add(0x1480)) as *mut u8) &= !2;
        let vtable = *(arg0 as *const u32);
        let target = *((vtable.wrapping_add(0xec)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let mut buf = [0u32; 3];
        let ans = f(arg0, buf.as_mut_ptr() as u32);
        let x = *((ans) as *const f32);
        let y = *((ans.wrapping_add(4)) as *const f32);
        let z = *((ans.wrapping_add(8)) as *const f32);
        let x2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(x), _mm_set_ss(x)));
        let y2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(y), _mm_set_ss(y)));
        let z2 = _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(z), _mm_set_ss(z)));
        let s01 = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(x2), _mm_set_ss(y2)));
        let s = _mm_cvtss_f32(_mm_add_ss(_mm_set_ss(s01), _mm_set_ss(z2)));
        let d = _mm_cvtss_f32(_mm_sqrt_ss(_mm_set_ss(s)));
        let t = f32::from_bits(*global::<u32>(0xfe87d0));
        // jbe: at-or-below or NaN takes the short path.
        if d > t {
            let b = *((this_obj.wrapping_add(0x1480)) as *const u8);
            let nb = (b & !8) | 3;
            *((this_obj.wrapping_add(0x145c)) as *mut u32) = 0x3f800000;
            *((this_obj.wrapping_add(0x1480)) as *mut u8) = nb;
            (ans & 0xffffff00) | (nb as u32)
        } else {
            *((this_obj.wrapping_add(0x1480)) as *mut u8) |= 0xb;
            *((this_obj.wrapping_add(0x145c)) as *mut u32) = 0x3c23d70b;
            ans
        }
    }
});
