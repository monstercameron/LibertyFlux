// original: 0x00a64b10 clamp_float_to_byte100
/// Clamps the truncated float argument into the byte at +0x2C8.
///
/// Converts with x86 truncation semantics (out-of-range and NaN yield the
/// indefinite value), then clamps the signed low byte to 0..100. Returns
/// nothing.
export!(thiscall, rw_00a64b10(this: u32, fbits: u32) -> u32 {
    let f = f32::from_bits(fbits);
    let t = if f < 2147483648.0 && f >= -2147483648.0 {
        f as i32
    } else {
        i32::MIN
    };
    let dl = t as i8;
    let v = if dl <= 0 {
        0u8
    } else if dl < 100 {
        dl as u8
    } else {
        100u8
    };
    unsafe {
        *((this + 0x2C8) as *mut u8) = v;
    }
    0
});
