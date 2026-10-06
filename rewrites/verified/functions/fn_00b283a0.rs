// original: 0x00b283a0 slot_record_lookup_limit

/// Copies a row's record under a float limit chosen by an index callee.
///
/// Calls the index callee with `a0`; the answer selects a 16-byte row.
/// Copies the row data's first three words (at data + 0x14) to `*out`, then,
/// when the row's byte count (compared signed) is positive, walks 32-byte
/// entries while the entry offset stays below the count: each entry's key
/// word is converted exactly to float and the walk stops at the first key
/// not below `lim` (NaN limit stops at once, as the original's branch exits
/// on unordered compare); otherwise the entry's three words replace `*out`.
/// Returns the third word when the count is not positive, the stopping key
/// shifted right by 31 (the original tests the key's sign bit into EAX)
/// when the limit stops the walk, or the past-end offset when the count
/// runs out. Cdecl, three stack words: key, float limit, output pointer.
lf_checker_rt::export!(cdecl, rw_00b283a0(a0: u32, lim: u32, out: u32) -> u32 {
    unsafe {
        const ROW_TABLE: u32 = 0x01657A10;
        const ROW_STRIDE: u32 = 16;
        const ROW_DATA: u32 = 4;
        const ROW_COUNT: u32 = 8;
        const HEAD_OFF: u32 = 0x14;
        const ENTRY_STRIDE: u32 = 0x20;
        const INDEX: u32 = 0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let idx = lf_checker_rt::callee_cdecl!(INDEX, u32, a0);
        let row = lf_checker_rt::relocated(ROW_TABLE) + idx.wrapping_mul(ROW_STRIDE);
        let data = rd32(row + ROW_DATA);
        let w2 = rd32(data + HEAD_OFF + 8);
        wr32(out, rd32(data + HEAD_OFF));
        wr32(out + 4, rd32(data + HEAD_OFF + 4));
        wr32(out + 8, w2);
        let count = rd32(row + ROW_COUNT) as i32;
        if count <= 0 {
            return w2;
        }
        let limf = f32::from_bits(lim);
        let mut p = data.wrapping_add(HEAD_OFF);
        loop {
            let key = rd32(p.wrapping_sub(HEAD_OFF));
            // Original: signed convert to double, add 2^32 when the sign
            // bit is set, round once to float. The double is exact either
            // way, so one exact widening plus one rounding matches it.
            let f = core::hint::black_box(key as f64) as f32;
            if !(limf > f) {
                return key >> 31;
            }
            wr32(out, rd32(p));
            wr32(out + 4, rd32(p + 4));
            wr32(out + 8, rd32(p + 8));
            p = p.wrapping_add(ENTRY_STRIDE);
            let off = p.wrapping_sub(data).wrapping_sub(HEAD_OFF) as i32;
            if !(off < count) {
                return off as u32;
            }
        }
    }
});
