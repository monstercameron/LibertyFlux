// original: 0x00969D80 sample_timing_profile

/// Reads a signed 16-bit-grid sample from a 120 by 120 table at twice the
/// flattened index, adds the global bias, and returns the float in ST0.
/// Either unsigned coordinate at or above 120 returns positive zero.
lf_checker_rt::export!(stdcall, rw_00969d80(x: u32, y: u32) -> f32 {
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    const LIMIT: u32 = 120;
    const TABLE_GLOBAL: u32 = 0x01218558;
    const BIAS_GLOBAL: u32 = 0x00e7ca00;
    if x >= LIMIT || y >= LIMIT {
        return 0.0;
    }
    unsafe {
        let flat_index = x.wrapping_add(y.wrapping_mul(LIMIT));
        let sample = lf_checker_rt::global::<i8>(TABLE_GLOBAL)
            .add(flat_index.wrapping_mul(2) as usize)
            .read() as i32;
        let bias = (lf_checker_rt::global::<f32>(BIAS_GLOBAL)).read_unaligned();
        add(sample as f32, bias)
    }
});
