// original: 0x00BEE0C0 convert_task_rgba_channels

/// Convert four float channels to byte fields. Each input f32 is multiplied
/// by the exact scale `127.0` in input-times-scale order, then truncated toward
/// zero as a signed 32-bit integer; NaN and out-of-range conversions produce
/// `i32::MIN`, matching the original conversion instruction. The low byte of
/// each result is written to `this + 0x0C` through `+0x0F`; EAX returns the full
/// signed conversion of the fourth channel. This is a thiscall with one pointer
/// argument to four consecutive f32 values.
lf_checker_rt::export!(thiscall, rw_00BEE0C0(this: u32, channels: u32) -> u32 {
    const CHANNEL_OFFSETS: [u32; 4] = [0, 4, 8, 12];
    const OUTPUT_OFFSETS: [u32; 4] = [0x0C, 0x0D, 0x0E, 0x0F];
    const SCALE: f32 = f32::from_bits(0x42FE0000);
    const INT_MIN: i32 = i32::MIN;
    const I32_UPPER_EXCLUSIVE: f32 = 2_147_483_648.0;
    const I32_LOWER_INCLUSIVE: f32 = -2_147_483_648.0;

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }
    #[inline(always)]
    fn multiply_input_by_scale(value: f32) -> f32 {
        core::hint::black_box(value) * core::hint::black_box(SCALE)
    }
    #[inline(always)]
    fn truncate_as_x86_i32(value: f32) -> i32 {
        if value.is_nan() || value >= I32_UPPER_EXCLUSIVE || value < I32_LOWER_INCLUSIVE {
            INT_MIN
        } else {
            k3probe(1, false, value.trunc()) as i32
        }
    }

    unsafe {
        let mut last_result = 0i32;
        for index in 0..CHANNEL_OFFSETS.len() {
            let scaled = multiply_input_by_scale(read_f32(channels + CHANNEL_OFFSETS[index]));
            last_result = truncate_as_x86_i32(scaled);
            write_u8(this + OUTPUT_OFFSETS[index], last_result as u8);
        }
        last_result as u32
    }
});
