// original: 0x0089ca40 audio_bsearch_records (proposed)

/// Binary search over a flat keyed record array, returning the match.
///
/// `key` is the sought key (compared UNSIGNED: the search uses `ja`/`jb`),
/// `base` is the record array base, and `count` is the record count.
/// Records are 10 bytes (`RECORD_SIZE`) with a 32-bit key at `+4`
/// (`KEY_OFF`); the array is searched as if sorted ascending by key.
///
/// The midpoint is the original's `(hi + lo - sign) / 2` signed-halving loop
/// (`cdq; sub; sar`). Unlike its sibling at 0x0089c860 there is no walk-back:
/// whichever match the search lands on is returned. Returns the record
/// address, or 0 when the count is 0 or the key is absent.
///
/// Original: 0x0089ca40 (cdecl, three stack words; no calls, no globals).
lf_checker_rt::export!(cdecl, rw_0089ca40(key: u32, base: u32, count: u32) -> u32 {
    unsafe {
        const RECORD_SIZE: u32 = 10;
        const KEY_OFF: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mut hi = (count as i32).wrapping_sub(1);
        if hi < 0 {
            return 0;
        }
        let mut lo: i32 = 0;
        loop {
            // Signed halving exactly as the original: (hi+lo-sign)/2.
            let sum = hi.wrapping_add(lo);
            let mid = sum.wrapping_sub(sum >> 31) >> 1;
            let probe = rd32(base.wrapping_add((mid as u32).wrapping_mul(RECORD_SIZE)) + KEY_OFF);
            if key > probe {
                lo = mid.wrapping_add(1);
            } else if key < probe {
                hi = mid.wrapping_sub(1);
            } else {
                return base.wrapping_add((mid as u32).wrapping_mul(RECORD_SIZE));
            }
            if lo > hi {
                return 0;
            }
        }
    }
});
