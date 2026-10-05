// original: 0x008ef950 clamped_call_to_int
/// Clamp a record's centred xor byte, run the helper, return an integer.
///
/// Centres the xor of the bytes at +6 and +4 around 127.5, scales it,
/// clamps it into [-1, 1], passes it with 0.25 to the helper (direct call,
/// stubbed by the checker), and converts the helper's answer times 127.5
/// to an integer with truncation (out-of-range and NaN yield 0x80000000).
export!(cdecl, rw_008ef950(p: u32) -> u32 {
    unsafe {
        let cl = *((p + 6) as *const u8) ^ *((p + 4) as *const u8);
        let lo = *global::<f32>(0xFE8D94);
        let centre = *global::<f32>(0xFE8BC8);
        let scale = *global::<f32>(0xE837F4);
        let hi = *global::<f32>(0xFE88E8);
        let x = (cl as f32 - centre) * scale;
        let v = if lo > x {
            lo
        } else if x > hi {
            hi
        } else {
            x
        };
        let r = callee_cdecl!(1, f32, v.to_bits(), 0x3E800000);
        let z = r * centre;
        let t = z.trunc();
        if t.is_nan() || t < -2147483648.0 || t >= 2147483648.0 {
            0x80000000u32
        } else {
            (t as i32) as u32
        }
    }
});
