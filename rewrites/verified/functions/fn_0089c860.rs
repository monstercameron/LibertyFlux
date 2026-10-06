// original: 0x0089c860 audio_bsearch_header_first (proposed)

/// Binary search over a keyed record array reached through a header, returning
/// the FIRST record with the key.
///
/// `header` points to a header whose dword at `+0` is the record array base
/// and whose 16-bit word at `+8` is the record count. Records are 14 bytes
/// (`RECORD_SIZE`), each holding a 32-bit key at `+8` (`KEY_OFF`); the array
/// is searched as if sorted ascending by key. `key` is the sought key
/// (compared UNSIGNED: the search uses `ja`/`jb`).
///
/// The midpoint is the original's `(hi + lo - sign) / 2` signed-halving loop
/// (`cdq; sub; sar`), exact here since both bounds stay small and
/// non-negative. On a match the scan walks back by whole records while the
/// previous record starts at or above the base (unsigned) and holds the same
/// key, so the earliest match wins. Returns the record address, or 0 when the
/// count is 0, the key is absent, or the match address itself is null.
///
/// Original: 0x0089c860 (cdecl, two stack words; no calls, no globals).
lf_checker_rt::export!(cdecl, rw_0089c860(header: u32, key: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x8;
        const RECORD_SIZE: u32 = 14;
        const KEY_OFF: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        let count = rd16(header + COUNT_OFF) as i32;
        let mut hi = count.wrapping_sub(1);
        if hi < 0 {
            return 0;
        }
        let base = rd32(header + BASE_OFF);
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
                let mut hit = base.wrapping_add((mid as u32).wrapping_mul(RECORD_SIZE));
                if hit == 0 {
                    return 0;
                }
                // Walk back to the first record with this key (`jb`: unsigned).
                loop {
                    let prev = hit.wrapping_sub(RECORD_SIZE);
                    if prev < base {
                        return hit;
                    }
                    if rd32(prev + KEY_OFF) != rd32(hit + KEY_OFF) {
                        return hit;
                    }
                    hit = prev;
                }
            }
            if lo > hi {
                return 0;
            }
        }
    }
});
