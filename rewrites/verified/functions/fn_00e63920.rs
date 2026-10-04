// original: 0x00e63920 store_ratio_a
/// Divide two global single-precision values, store the ratio.
///
/// Computes `[0x1036F1C] / [0x1036F20]` with SSE single-precision division
/// and stores the result at 0x11A8914. The division runs through one SSE
/// divide instruction so special values (zero, subnormal, infinity, NaN)
/// behave exactly as in the original. Leaves no meaningful return value.
export!(cdecl, rw_00e63920() -> u32 {
    unsafe {
        let ratio = sse_div(
            *global::<f32>(0x1036F1C),
            *global::<f32>(0x1036F20),
        );
        *global::<f32>(0x11A8914) = ratio;
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
