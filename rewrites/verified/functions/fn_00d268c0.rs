// original: 0x00d268c0 ped_stat_scale_clamp (proposed)

/// Scale a callee-provided float and clamp negatives to zero.
///
/// Calls the stat helper (intercepted) with `arg`, then computes
/// `x = f * 25.0 - 15.0` in that operation order. A negative result is
/// replaced with 0.0; a NaN is kept as is (the original's jump-if-below-or-
/// equal keeps unordered values).
///
/// Original: 0x00D268C0 (cdecl, one stack argument). Returns the float in ST0.
lf_checker_rt::export!(cdecl, rw_00d268c0(arg: u32) -> f32 {
    unsafe {
        const SCALE_BITS: u32 = 0x41c8_0000; // 25.0f
        const SHIFT_BITS: u32 = 0x4170_0000; // 15.0f
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let f: f32 = lf_checker_rt::callee_cdecl!(1, f32, arg);
        let x = sub(mul(f, f32::from_bits(SCALE_BITS)), f32::from_bits(SHIFT_BITS));
        if x < 0.0 { 0.0 } else { x }
    }
});
