// original: 0x00C37C80 decode_quant_vector

/// Convert three unsigned 16-bit samples into a three-component floating
/// vector, scale each component by its corresponding factor, then add the
/// corresponding bias. The six cdecl stack arguments are, in order, the
/// writable output vector, pointers to the three samples, a three-float bias
/// vector, and a three-float scale vector. Each converted sample is multiplied
/// by the fixed game-unit scale before the per-component scale and bias steps.
/// Floating operations preserve the original single-precision order.
lf_checker_rt::export!(cdecl, rw_00c37c80(
    output: *mut u8,
    sample_x: *const u8,
    sample_y: *const u8,
    sample_z: *const u8,
    bias: *const u8,
    scale: *const u8,
) -> () {
    const UNIT_SCALE_BITS: u32 = 0x3780_0000;
    #[inline(never)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(never)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let samples = [sample_x, sample_y, sample_z];
        let mut component = 0usize;
        while component < 3 {
            let sample = samples[component].cast::<u16>().read_unaligned() as f32;
            let unit_value = mul_f32(sample, f32::from_bits(UNIT_SCALE_BITS));
            let component_scale = scale.add(component * 4).cast::<f32>().read_unaligned();
            let scaled_value = mul_f32(component_scale, unit_value);
            let translated_value = add_f32(
                bias.add(component * 4).cast::<f32>().read_unaligned(),
                scaled_value,
            );
            output.add(component * 4).cast::<f32>().write_unaligned(translated_value);
            component += 1;
        }
    }
});
