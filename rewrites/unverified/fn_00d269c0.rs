// original: 0x00d269c0 ped_stat_scale_bias (proposed)

/// Scale and bias a callee-provided float: `f * 0.6 + 0.4`, in that order.
///
/// Calls the stat helper (intercepted) with `arg` and applies the linear map.
/// No clamping: infinities and NaNs propagate per IEEE rules.
///
/// Original: 0x00D269C0 (cdecl, one stack argument). Returns the float in ST0.
lf_checker_rt::export!(cdecl, rw_00d269c0(arg: u32) -> f32 {
    unsafe {
        const SCALE_BITS: u32 = 0x3f19_999a; // 0.6f
        const BIAS_BITS: u32 = 0x3ecc_cccd; // 0.4f
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let f: f32 = lf_checker_rt::callee_cdecl!(1, f32, arg);
        add(mul(f, f32::from_bits(SCALE_BITS)), f32::from_bits(BIAS_BITS))
    }
});
