// original: 0x00D64130 convert_color_channel_to_float

/// Convert a signed integer channel to the game's scaled single-precision form.
///
/// The input is an `i32` value from the stack. It is converted to `f32`, then
/// the original image's additive bias is added, the result is multiplied by
/// the image's scale, and the final bias is subtracted. The three constants
/// are read through the relocated image so the rewrite follows the executable
/// data rather than baking in the current values. The result is returned in
/// the x87 floating-point channel with the original operation order.
lf_checker_rt::export!(cdecl, rw_convert_color_channel_to_float(channel: i32) -> f32 {
    unsafe {
        const ADDITIVE_BIAS_VA: u32 = 0x00FE8830;
        const SCALE_VA: u32 = 0x00E83174;
        const SUBTRACTIVE_BIAS_VA: u32 = 0x00FE8C78;

        #[inline(always)]
        unsafe fn read_image_f32(file_va: u32) -> f32 {
            f32::from_bits(unsafe {
                lf_checker_rt::global::<u32>(file_va).read_unaligned()
            })
        }

        #[inline(always)]
        fn add_in_source_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        fn multiply_in_source_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn subtract_in_source_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        let channel_as_float = channel as f32;
        let additive_bias = read_image_f32(ADDITIVE_BIAS_VA);
        let scaled_input = add_in_source_order(channel_as_float, additive_bias);
        let scale = read_image_f32(SCALE_VA);
        let multiplied = multiply_in_source_order(scaled_input, scale);
        let subtractive_bias = read_image_f32(SUBTRACTIVE_BIAS_VA);
        subtract_in_source_order(multiplied, subtractive_bias)
    }
});
