// original: 0x009693F0 sum_timing_sample_triplets

/// Adds three groups of three floats from the object’s timing sample region.
/// For index `i`, the samples lie at `0x2764 + 4*i + [-4,0,4,32,36,40,68,72,76]`.
/// Accumulation follows the original scalar SSE order, then multiplies by
/// the global sample scale and returns the result in ST0.
lf_checker_rt::export!(thiscall, rw_009693f0(this: u32, index: u32) -> f32 {
    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits((address as *const u32).read_unaligned())
    }
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }

    unsafe {
        const SAMPLE_BASE: u32 = 0x2764;
        const ROW_STRIDE: u32 = 0x24;
        const SCALE_GLOBAL: u32 = 0x00fe87a0;
        let first = this.wrapping_add(SAMPLE_BASE).wrapping_add(index.wrapping_mul(4));
        let mut total = 0.0f32;
        for row in 0..3u32 {
            let row_base = first.wrapping_add(row * ROW_STRIDE);
            total = add(total, read_float(row_base.wrapping_sub(4)));
            total = add(total, read_float(row_base));
            total = add(total, read_float(row_base.wrapping_add(4)));
        }
        let scale = read_float(lf_checker_rt::global::<u32>(SCALE_GLOBAL) as u32);
        multiply(total, scale)
    }
});
