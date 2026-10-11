// original: 0x00638650 rage::ptxEffectRuleStd::vf31

/// Convert the receiver's signed 32-bit sample index plus one to `f32`, then
/// multiply by the receiver's scale. A negative index returns exactly `-1.0`
/// without reading the scale. The increment wraps as a signed 32-bit value;
/// arithmetic order follows the original conversion-then-multiply sequence.
lf_checker_rt::export!(thiscall, rw_00638650(this: u32) -> f32 {
    unsafe {
        const SAMPLE_COUNT: u32 = 0x12C;
        const SAMPLE_SCALE: u32 = 0x124;
        const NEGATIVE_RESULT: f32 = f32::from_bits(0xBF80_0000);

        let signed_index = (this.wrapping_add(SAMPLE_COUNT) as *const i32).read_unaligned();
        if signed_index < 0 {
            return NEGATIVE_RESULT;
        }

        let incremented_index = signed_index.wrapping_add(1);
        let converted_index = incremented_index as f32;
        let scale = (this.wrapping_add(SAMPLE_SCALE) as *const f32).read_unaligned();
        let converted_index = core::hint::black_box(converted_index);
        let scale = core::hint::black_box(scale);
        converted_index * scale
    }
});
