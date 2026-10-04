// original: 0x00ae0cc0 ui_scaled_byte_push
/// Send two control bytes downstream, the first scaled to a float.
///
/// Converts the byte at `+0x22` to float, multiplies it by the global scale
/// factor with SSE (`mulss`, bit-exact) and sends the result to the float sink;
/// then sends the zero-extended byte at `+0x23` to the integer sink.
/// Returns the integer sink's answer.
export!(thiscall, rw_00ae0cc0(this_ptr: u32) -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_cvtss_f32, _mm_cvtsi32_si128, _mm_cvtepi32_ps, _mm_mul_ss,
            _mm_set_ss,
        };
        let b = ((this_ptr + 0x22) as *const u8).read() as i32;
        let f = _mm_cvtss_f32(_mm_mul_ss(
            _mm_cvtepi32_ps(_mm_cvtsi32_si128(b)),
            _mm_set_ss(*global::<f32>(0xFE86E8)),
        ));
        callee_cdecl!(1, u32, f.to_bits());
        let b2 = ((this_ptr + 0x23) as *const u8).read() as u32;
        callee_cdecl!(2, u32, b2)
    }
});
