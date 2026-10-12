// original: 0x008C14E0 audio_param_scaled_to_int

/// Scale one audio parameter by 1000 and truncate it to an integer.
///
/// Loads the float at `TABLE + index * 12` past the object, multiplies it by
/// the scale factor held in the image (1000.0, loaded from the image, not
/// written as a literal), and converts with the x87 truncating 64-bit store:
/// NaN and values outside the signed 64-bit range give the integer
/// indefinite, whose low word is 0; anything else truncates toward zero and
/// the low 32 bits are returned. Thiscall, object in ECX, index popped
/// from the stack.
lf_checker_rt::export!(thiscall, rw_008C14E0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0xC0;
        const SCALE: u32 = 0x00FE_8C58;
        const STRIDE: u32 = 3;
        let slot = this
            .wrapping_add(TABLE)
            .wrapping_add(index.wrapping_mul(STRIDE).wrapping_mul(4));
        let value = f32::from_bits((slot as *const u32).read_unaligned());
        let scale = f32::from_bits(
            (lf_checker_rt::relocated(SCALE) as *const u32).read_unaligned(),
        );
        let first = core::hint::black_box(value);
        let second = core::hint::black_box(scale);
        let scaled = first * second;
        if scaled.is_nan() || scaled >= 9223372036854775808.0 || scaled < -9223372036854775808.0 {
            0
        } else {
            (scaled as i64) as u32
        }
    }
});
