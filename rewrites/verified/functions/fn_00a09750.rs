// original: 0x00a09750 mission_cleanup_intern (proposed)
/// Find or insert a mission-cleanup record in the overflow table.
///
/// `spec` points at a descriptor (dword at +4 is the payload, byte at +0x94
/// selects the table half: zero scans rows 0..0x78, nonzero rows 0x78..0xc8
/// of the 0x2c-byte records at `this + 0x2c04`). If a row matches `key` at
/// +4, `tag` (low byte) at +0 and the payload at +8, it is returned. Else
/// the last all-zero-tag row seen is filled with (`tag`, `key`, payload)
/// and the payload returned. If no row is empty, the original falls through
/// with the setup arithmetic in eax (low byte overwritten by `tag`), which
/// is reproduced exactly. Thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00a09750(this: u32, key: u32, tag: u32, spec: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x2c04;
        const STRIDE: u32 = 0x2c;
        const HALF: u32 = 0x78;
        const ROWS: u32 = 0xc8;
        let tag_lo = (tag & 0xff) as u8;
        let sel = ((spec + 0x94) as *const u8).read();
        let (mut lo, hi) = if sel == 0 { (0u32, HALF) } else { (HALF, ROWS) };
        let payload = ((spec + 4) as *const u32).read_unaligned();
        if lo >= hi {
            return 0;
        }
        let mut rec = this + TABLE + lo * STRIDE;
        let mut empty: i32 = -1;
        while lo < hi {
            let k = ((rec + 4) as *const u32).read_unaligned();
            let t = (rec as *const u8).read();
            if k == key && t == tag_lo {
                let p = ((rec + 8) as *const u32).read_unaligned();
                if p == payload {
                    return p;
                }
            }
            if t == 0 {
                empty = lo as i32;
            }
            lo += 1;
            rec += STRIDE;
        }
        if empty == -1 {
            let base = if sel == 0 { 0u32 } else { HALF * STRIDE };
            return (base & 0xffff_ff00) | (tag_lo as u32);
        }
        let at = this + TABLE + (empty as u32) * STRIDE;
        ((at + 4) as *mut u32).write_unaligned(key);
        (at as *mut u8).write(tag_lo);
        ((at + 8) as *mut u32).write_unaligned(payload);
        payload
    }
});
