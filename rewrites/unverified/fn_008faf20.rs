// original: 0x008FAF20 stream_scaled_time_quantize
/// Quantize the product of two global floats to a 32-bit count.
///
/// Multiplies the two global floats (in that order), then converts
/// the product to a 64-bit integer with truncation toward zero (the
/// original switches the x87 control word to chop mode for one
/// `fistp`) and returns the low 32 bits. Out-of-range results,
/// infinities and NaN convert to the indefinite value 0x80000000,
/// exactly like `fistp`. Cdecl, no arguments.
export!(cdecl, rw_008faf20() -> u32 {
    unsafe {
        const RATE: u32 = 0x11735dc;
        const SCALE: u32 = 0xfe8c58;
        const I64_MAX_PLUS_1: f32 = 9223372036854775808.0;
        let a = f32::from_bits(*global::<u32>(RATE));
        let b = f32::from_bits(*global::<u32>(SCALE));
        let p = core::hint::black_box(a) * core::hint::black_box(b);
        let q: i64 = if p.is_nan() || p >= I64_MAX_PLUS_1 || p < -I64_MAX_PLUS_1 {
            i64::MIN
        } else {
            p as i64
        };
        q as u32
    }
});
