// original: 0x008ef8c0 scaled_call_to_int
/// Scale a record's xor byte through the helper and return an integer.
///
/// Takes the xor of the bytes at +6 and +4, scales it by a constant factor,
/// passes it with 0.25 to the helper (direct call, stubbed by the checker),
/// and converts the helper's float answer times a second factor to an
/// integer with truncation (out-of-range and NaN yield 0x80000000, exactly
/// like the original's `cvttss2si`).
export!(cdecl, rw_008ef8c0(p: u32) -> u32 {
    unsafe {
        let cl = *((p + 6) as *const u8) ^ *((p + 4) as *const u8);
        let y = (cl as f32) * *global::<f32>(0xFE86E8);
        let r = callee_cdecl!(1, f32, y.to_bits(), 0x3E800000);
        let z = r * *global::<f32>(0xE83800);
        let t = z.trunc();
        if t.is_nan() || t < -2147483648.0 || t >= 2147483648.0 {
            0x80000000u32
        } else {
            (t as i32) as u32
        }
    }
});
