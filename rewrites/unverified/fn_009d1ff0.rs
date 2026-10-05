// original: 0x009D1FF0 pool_index_search (proposed)
//
/// Linear search for a dword in a bounded slice of an array.
///
/// Searches `table[start .. start+count]` for the dword stored at `*needle`,
/// returning the first matching index or -1. Every bound is SIGNED: a
/// negative `start` or `count`, `start >= limit` (`[this+4]`), a wrapped or
/// over-long `end = start+count` above `limit`, and an empty range all yield
/// -1 without reading the array. Thiscall, three stack words
/// `(needle, start, count)`.
lf_checker_rt::export!(thiscall, rw_009D1FF0(this: u32, needle: u32, start: u32, count: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const LIMIT: u32 = 0x04;
        const NOT_FOUND: u32 = 0xffffffff;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let limit = rd(this.wrapping_add(LIMIT));
        if (start as i32) < 0 || (start as i32) >= (limit as i32) {
            return NOT_FOUND;
        }
        if (count as i32) < 0 {
            return NOT_FOUND;
        }
        let end = start.wrapping_add(count);
        if (end as i32) > (limit as i32) || (start as i32) >= (end as i32) {
            return NOT_FOUND;
        }
        let want = rd(needle);
        let base = rd(this.wrapping_add(TABLE));
        let mut i = start;
        loop {
            if rd(base.wrapping_add(i.wrapping_mul(4))) == want {
                return i;
            }
            i = i.wrapping_add(1);
            if (i as i32) >= (end as i32) {
                return NOT_FOUND;
            }
        }
    }
});
