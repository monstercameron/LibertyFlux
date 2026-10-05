// original: 0x009D39A0 pair_scan_match (proposed)
//
/// Scans record pairs for the first one a predicate accepts.
///
/// Walks the `count` (`[this+0x0c]`, 16-bit) 8-byte pairs at `[this+0x08]`,
/// calling the predicate (callee) with each pair's first dword and `key`.
/// Returns 1 in `al` on the first zero answer, else 0; the upper 24 bits of
/// `eax` are whatever the last predicate call left behind (zero when the
/// loop never ran), so only `al` is compared. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_009D39A0(this: u32, key: u32) -> u32 {
    unsafe {
        const PAIRS: u32 = 0x08;
        const COUNT: u32 = 0x0c;
        const PRED: u32 = 1;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let pairs = rd(this.wrapping_add(PAIRS));
        let mut i = 0u32;
        loop {
            let first = rd(pairs.wrapping_add(i.wrapping_mul(8)));
            let ans: u32 = lf_checker_rt::callee_cdecl!(PRED, u32, first, key);
            if ans == 0 {
                return 1;
            }
            i += 1;
            if (i as i32) >= (count as i32) {
                return 0;
            }
        }
    }
});
