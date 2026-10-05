// original: 0x008efa20 clamped_call
/// Clamp a record's centred xor byte and run the helper, returning its float.
///
/// Same input shaping as [`rw_008ef950`] but returns the helper's float
/// answer untouched.
export!(cdecl, rw_008efa20(p: u32) -> f32 {
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
        callee_cdecl!(1, f32, v.to_bits(), 0x3E800000)
    }
});
