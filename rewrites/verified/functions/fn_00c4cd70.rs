// original: 0x00C4CD70 convert_motion_parameters (proposed)
/// Store the first and third stack floats bit-for-bit in the image globals,
/// and store the low word of the middle float truncated toward zero through
/// the function's temporary x87 control-word mode. The original's qword
/// conversion yields the x87 indefinite value when the input is not finite
/// or lies outside signed 64-bit range, so its stored low word is zero. The
/// cdecl method takes three single-precision values and returns no value.
lf_checker_rt::export!(cdecl, rw_c4cd70(first_bits: u32, middle_bits: u32, third_bits: u32) -> u32 {
    unsafe {
        const FIRST_GLOBAL_VA: u32 = 0x0104_909c;
        const MIDDLE_GLOBAL_VA: u32 = 0x0104_90a0;
        const THIRD_GLOBAL_VA: u32 = 0x0104_90a4;
        const TWO_TO_63: f64 = 9_223_372_036_854_775_808.0;

        let middle = f32::from_bits(middle_bits);
        let converted = if !middle.is_finite()
            || (middle as f64) >= TWO_TO_63
            || (middle as f64) < -TWO_TO_63
        {
            i64::MIN
        } else {
            middle as i64
        };
        lf_checker_rt::global::<u32>(FIRST_GLOBAL_VA).write_unaligned(first_bits);
        lf_checker_rt::global::<u32>(MIDDLE_GLOBAL_VA).write_unaligned(converted as u32);
        lf_checker_rt::global::<u32>(THIRD_GLOBAL_VA).write_unaligned(third_bits);
        0
    }
});
