// original: 0x00b05430 for_each_pair_calling_worker

/// Calls the worker once per 8-byte record in `[cur, end)`.
///
/// Each call receives the record address, its two words, and `tag`, and the
/// last worker answer is returned. The third stack word is never read by the
/// original; the tag is the fourth. Note: when the range is empty the original
/// returns whatever the caller left in EAX; that value is not an input the
/// checker can script, so the rewrite returns 0 there and the contract only
/// exercises non-empty ranges.
export!(cdecl, rw_00b05430(cur: *mut u32, end: *const u32, _unused: u32, tag: u32) -> u32 {
    unsafe {
        let mut last = 0u32;
        let mut p = cur as u32;
        let stop = end as u32;
        if p != stop {
            loop {
                let w0 = *(p as *const u32);
                let w1 = *((p.wrapping_add(4)) as *const u32);
                last = callee_cdecl!(1, u32, p, w0, w1, tag);
                p = p.wrapping_add(8);
                if p == stop {
                    break;
                }
            }
        }
        last
    }
});
