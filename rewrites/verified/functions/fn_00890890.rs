// original: 0x00890890 audio_float_ptr_to_millis
/// Scale the float behind this object's +0x5c pointer by 1000, truncated.
///
/// A null pointer yields 0. Otherwise the pointed-to float is multiplied by
/// 1000.0 and converted with truncation toward zero; NaN and out-of-range
/// results yield `0x80000000`, exactly like the original's `cvttss2si`.
export!(thiscall, rw_00890890(this: u32) -> u32 {
    const SCALE: f32 = f32::from_bits(0x447A_0000); // 1000.0, measured
    unsafe {
        let ptr = ((this + 0x5c) as *const u32).read();
        if ptr == 0 {
            return 0;
        }
        let scaled = (ptr as *const f32).read() * SCALE;
        // Match cvttss2si: a plain `as` cast saturates, so handle the
        // out-of-range and NaN cases explicitly.
        if scaled.is_nan() || scaled >= 2147483648.0 || scaled <= -2147483648.0 {
            0x80000000
        } else {
            (scaled as i32) as u32
        }
    }
});
