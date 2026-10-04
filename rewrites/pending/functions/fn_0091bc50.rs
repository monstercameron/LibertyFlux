// original: 0x0091BC50 text_scale_factor
/// Scale one per-index table float by the global text factor.
///
/// Queries the index helper, loads the float at index 72 of the table at
/// `0x0119BF20`, multiplies it by the global float at `0x00E85F6C` with
/// `mulss`, and returns the product on `ST0`. The multiply uses SSE
/// intrinsics so it is bit-exact with the original.
export!(cdecl, rw_0091bc50() -> f32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let idx: u32 = callee_cdecl!(1, u32,);
        let t = f32::from_bits(
            *((relocated(0x119BF20) + idx.wrapping_mul(9).wrapping_mul(8)) as *const u32));
        let g = f32::from_bits(*global::<u32>(0xE85F6C));
        _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(t), _mm_set_ss(g)))
    }
});
