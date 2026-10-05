// original: 0x00887DA0 stream_sorted_find (proposed)

/// Binary search for a key over a strided row table.
///
/// `table` holds rows of 16 bytes with the sort key at row offset 8;
/// `hi` is the last signed row index (`hi` below zero, or a null table,
/// finds nothing). The midpoint is `(lo + hi) / 2` through the original's
/// add-with-carry, arithmetic-shift sequence; rows compare unsigned. The
/// result is the matching row index, or 0xffff_ffff when no row matches.
///
/// Original: 0x00887DA0 (stdcall, three stack words).
lf_checker_rt::export!(stdcall, rw_00887DA0(key: u32, table: u32, hi: u32) -> u32 {
    unsafe {
        const ROW: u32 = 16;
        const KEY_OFF: u32 = 8;
        const MISS: u32 = 0xffff_ffff;
        if table == 0 {
            return MISS;
        }
        let mut lo: i32 = 0;
        let mut hi = hi as i32;
        if hi < 0 {
            return MISS;
        }
        loop {
            // (an instruction of the original); cdq; (an instruction of the original); (an instruction of the original).
            let sum = (lo as u32).wrapping_add(hi as u32);
            let mid = sum.wrapping_add(sum >> 31) as i32 >> 1;
            let at = table
                .wrapping_add((mid as u32).wrapping_mul(2).wrapping_mul(8))
                .wrapping_add(KEY_OFF);
            let entry = (at as *const u32).read_unaligned();
            if key > entry {
                lo = mid.wrapping_add(1);
            } else if key == entry {
                return mid as u32;
            } else {
                hi = mid.wrapping_sub(1);
            }
            if lo > hi {
                return MISS;
            }
        }
    }
});
