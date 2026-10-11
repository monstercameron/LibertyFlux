// original: 0x00c3d900 train_set_level_byte_and_mirror (proposed)
/// Store a truncated float level on an object and its looked-up partner.
///
/// `obj` points to the object, `fbits` is a float bit pattern. Converts
/// the float with truncation toward zero exactly like `cvttss2si`
/// (NaN, infinities and out-of-range values yield 0x80000000, unlike a
/// saturating cast) and stores the low byte at `obj+0xe6f`. Then calls
/// helper id 1 (cdecl, obj); when it returns non-null the same byte is
/// also stored at `ret+0x27`. Returns the helper's answer.
///
/// Original: 0x00c3d900 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c3d900(obj: u32, fbits: u32) -> u32 {
    unsafe {
        const LEVEL_SELF: u32 = 0xe6f;
        const LEVEL_PARTNER: u32 = 0x27;
        const LOOKUP: u32 = 1;
        const I32_MAX_PLUS_1: f32 = 2147483648.0;
        const I32_MIN_F: f32 = -2147483648.0;
        let f = f32::from_bits(fbits);
        // Match cvttss2si: invalid (NaN or out of range) -> 0x80000000.
        let v: i32 = if f.is_nan() || f >= I32_MAX_PLUS_1 || f < I32_MIN_F {
            core::hint::black_box(0x80000000u32) as i32
        } else {
            core::hint::black_box(f) as i32
        };
        let bl = (v & 0xff) as u8;
        ((obj + LEVEL_SELF) as *mut u8).write(bl);
        let got: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, obj);
        if got != 0 {
            ((got + LEVEL_PARTNER) as *mut u8).write(bl);
        }
        got
    }
});
