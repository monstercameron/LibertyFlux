// original: 0x00abc1d0 for_each_call_worker

/// Invoke the element worker (stubbed) for each word in a half-open range.
///
/// Walks the pointer from `begin` to `end` four bytes at a time, calling the
/// worker with the element address, the element value and the carried extra
/// argument, which is the fourth stack word: the third word is never read.
/// Returns the last worker answer. Note: an empty range returns whatever the
/// caller left in EAX; the rewrite returns 0 there since entry registers are
/// unobservable from safe Rust.
export!(cdecl, rs64_abc1d0(begin: u32, end: u32, _unused: u32, x: u32) -> u32 {
    unsafe {
        let mut ans = 0u32;
        let mut p = begin;
        if p != end {
            loop {
                let v = *(p as *const u32);
                ans = callee_cdecl!(0, u32, p, v, x);
                p = p.wrapping_add(4);
                if p == end {
                    break;
                }
            }
        }
        ans
    }
});
