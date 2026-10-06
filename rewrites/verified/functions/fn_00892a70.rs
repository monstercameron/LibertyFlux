// original: 0x00892A70 audsound_slot_bsearch
/// Binary-searches the slot table for `key`, returning its index or -1.
///
/// `table` points at 16-byte entries whose sort key sits at entry offset 8.
/// `hi` is the last index to consider (signed: negative means empty). The
/// midpoint is `(lo + hi) / 2` truncated toward zero on the wrapped 32-bit
/// sum; keys compare UNSIGNED (`key` above the entry key goes right, below
/// goes left, equal returns the index). The loop ends when `lo` passes `hi`
/// (signed). Returns -1 (all-ones) when the enable byte at `this+0x4d` is
/// clear, when `table` is null, when `hi` is negative, or when the key is
/// absent. Reads only; the only fault source is an out-of-range `hi`.
/// Original: 0x00892A70 (thiscall, three stack words: key, table, hi).
export!(thiscall, rw_00892A70(this: *mut u8, key: u32, table: u32, hi: i32) -> u32 {
    unsafe {
        const ENABLE: usize = 0x4d;
        const ENTRY_STRIDE: i32 = 16;
        const KEY_OFF: isize = 8;
        if *this.add(ENABLE) == 0 {
            return 0xffff_ffff;
        }
        if table == 0 {
            return 0xffff_ffff;
        }
        if hi < 0 {
            return 0xffff_ffff;
        }
        let mut lo: i32 = 0;
        let mut hi = hi;
        loop {
            // Truncated midpoint of the wrapped sum, as the original's
            // lea/cdq/sub/sar sequence computes it.
            let sum = lo.wrapping_add(hi);
            let mid = sum.wrapping_sub(sum >> 31) >> 1;
            let cur = *((table.wrapping_add((mid as u32).wrapping_mul(ENTRY_STRIDE as u32)) as *const u8)
                .offset(KEY_OFF) as *const u32);
            if key > cur {
                lo = mid.wrapping_add(1);
            } else if key < cur {
                hi = mid.wrapping_sub(1);
            } else {
                return mid as u32;
            }
            if lo > hi {
                return 0xffff_ffff;
            }
        }
    }
});
