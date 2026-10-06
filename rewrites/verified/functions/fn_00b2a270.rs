// original: 0x00b2a270 slot_row_smooth

/// Smooths one row's data in place with a halved neighbour sum.
///
/// `idx` selects a 16-byte row; the row's data pointer and byte count are
/// read from it. When `count - 32` (signed) is not above 32, returns that
/// difference and writes nothing. Otherwise, for offsets 0x20, 0x40, ...,
/// each below `count - 64`: adds the words 32 bytes below and 32 bytes above
/// the target (wrapping, in place so the lower word is the just-written
/// value), converts the sum exactly to float, halves it, truncates toward
/// zero to a 64-bit integer and stores the low 32 bits. The truncation
/// matches the original's x87 `fistp` with truncation rounding: NaN and
/// values at or above 2^63 store 0x80000000 (low word of the indefinite
/// value); values below -2^63 saturate to the same low word either way.
/// Always returns `count - 32` (the count is reloaded after the loop, so the
/// stored words are not reflected in the return value). Cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_00b2a270(idx: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_DATA: u32 = 4;
        const ROW_COUNT: u32 = 8;
        const HALF: f32 = 0.5;
        const TWO63: f32 = 9223372036854775808.0; // 2^63, exact
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Bit-exact truncation of a float to i64 with x87 fistp semantics.
        #[inline(always)]
        fn trunc_fistp(f: f32) -> i64 {
            if f.is_nan() || f >= TWO63 {
                i64::MIN
            } else {
                f as i64
            }
        }
        let row = lf_checker_rt::relocated(ROW_TABLE) + idx.wrapping_mul(ROW_STRIDE);
        let data = rd32(row + ROW_DATA);
        let count = rd32(row + ROW_COUNT) as i32;
        if count.wrapping_sub(0x20) <= 0x20 {
            return count.wrapping_sub(0x20) as u32;
        }
        let mut o = 0x20u32;
        loop {
            let a = rd32(data.wrapping_add(o).wrapping_sub(0x20))
                .wrapping_add(rd32(data.wrapping_add(o).wrapping_add(0x20)));
            // Original: signed convert to double, add 2^32 when the sign
            // bit is set, round once to float. The double is exact either
            // way, so one exact widening plus one rounding matches it.
            let d = core::hint::black_box(a as f64);
            let f = mul(d as f32, HALF);
            let q = trunc_fistp(f);
            ((data.wrapping_add(o)) as *mut u32).write_unaligned(q as u32);
            if !((o as i32) < count.wrapping_sub(0x40)) {
                break;
            }
            o = o.wrapping_add(0x20);
        }
        count.wrapping_sub(0x20) as u32
    }
});
