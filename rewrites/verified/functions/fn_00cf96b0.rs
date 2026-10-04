// original: 0x00cf96b0 climb_blend_factor_clamp (proposed)

/// Computes a clamped blend factor from a measured value. Reads the value
/// through the second argument, compares it against the low threshold (-0.6);
/// values at or below the threshold (including NaN) leave the output alone.
/// Above the threshold the output (through the fourth argument) becomes the
/// larger of the floor (-0.05) and `(value - threshold) * floor`, evaluated
/// in that order; a NaN intermediate propagates to the output. Only the
/// second and fourth of the five stack arguments are read.
///
/// Original: 0x00cf96b0 (stdcall, five stack words).
lf_checker_rt::export!(stdcall, rw_00cf96b0(_a: u32, in_value: u32, _b: u32, out_value: u32, _c: u32) -> u32 {
    unsafe {
        const THRESHOLD_ADDR: u32 = 0x00fe8d84;
        const FLOOR_ADDR: u32 = 0x00fe8d5c;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        let threshold = f32::from_bits((lf_checker_rt::global::<u32>(THRESHOLD_ADDR)).read_unaligned());
        let floor = f32::from_bits((lf_checker_rt::global::<u32>(FLOOR_ADDR)).read_unaligned());
        let x = f32::from_bits((in_value as *const u32).read_unaligned());
        if x > threshold {
            let scaled = mul(sub(x, threshold), floor);
            let clamped = if floor > scaled { floor } else { scaled };
            (out_value as *mut u32).write_unaligned(clamped.to_bits());
        }
        // The original returns its second stack word on the early-out path
        // and its fourth on the store path (leftover in eax either way).
        if x > threshold { out_value } else { in_value }
    }
});
