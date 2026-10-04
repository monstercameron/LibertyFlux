// original: 0x00e639d0 store_ratio_b
/// Divide two global single-precision values, store the ratio.
///
/// Computes `[0x10375A4] / [0x10375A8]` and stores the result at 0x11D6FFC.
/// See [`rw_00e63920`] for the shared reasoning.
export!(cdecl, rw_00e639d0() -> u32 {
    unsafe {
        let ratio = sse_div(
            *global::<f32>(0x10375A4),
            *global::<f32>(0x10375A8),
        );
        *global::<f32>(0x11D6FFC) = ratio;
        0
    }
});

/// Single-precision division through one SSE divide instruction.
///
/// Keeps special-value behaviour (zero, subnormal, infinity, NaN) bit-identical
/// to the original's `divss`, which a software or x87 division could not
/// guarantee.
#[target_feature(enable = "sse")]
unsafe fn sse_div(a: f32, b: f32) -> f32 {
    use core::arch::x86::{_mm_cvtss_f32, _mm_div_ss, _mm_set_ss};
    _mm_cvtss_f32(_mm_div_ss(_mm_set_ss(a), _mm_set_ss(b)))
}
