// original: 0x0091C7A0 global_float_product
/// Multiply two global floats and return the product on `ST0`.
///
/// Loads the single-precision values at `0x010366A8` and `0x00E85F6C`,
/// multiplies them with `mulss` and returns the bit-exact product. Takes no
/// arguments; entry registers are ignored. The multiply uses SSE intrinsics
/// so it is the same single-rounded instruction as the original.
export!(cdecl, rw_0091c7a0() -> f32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_mul_ss, _mm_set_ss};
        let a = f32::from_bits(*global::<u32>(0x10366A8));
        let b = f32::from_bits(*global::<u32>(0xE85F6C));
        _mm_cvtss_f32(_mm_mul_ss(_mm_set_ss(a), _mm_set_ss(b)))
    }
});
