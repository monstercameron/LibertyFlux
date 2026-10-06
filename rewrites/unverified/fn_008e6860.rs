// original: 0x008E6860 sift_callbacks (proposed)

/// Drive the sift callback down the heap positions.
///
/// `n` is the element count `(end - base) / 8` (signed). Counts below 2
/// return at once (the original leaves the entry `eax` untouched there, so
/// the contract compares no return value). Otherwise the sift callback
/// (callee 1, cdecl) is invoked for positions `(n - 2) / 2` down to 0 as
/// `(base, pos, n, words[pos * 2], words[pos * 2 + 1], ctx)`, where the two
/// words are the 8-byte element at `base + pos * 8`.
///
/// Original: 0x008E6860 (cdecl, three stack arguments).
lf_checker_rt::export!(cdecl, rw_008E6860(base: u32, end: u32, ctx: u32) -> u32 {
    unsafe {
        /// Element size shift: counts are byte lengths over 8.
        const ELEM_SHIFT: i32 = 3;
        /// Element size in bytes.
        const ELEM_SIZE: u32 = 8;
        /// Sift callback callee id.
        const SIFT: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let n = ((end.wrapping_sub(base)) as i32) >> ELEM_SHIFT;
        if n < 2 {
            return 0;
        }
        let mut i = ((n - 2) / 2) as u32;
        loop {
            let row = base.wrapping_add(i.wrapping_mul(ELEM_SIZE));
            let lo = rd32(row);
            let hi = rd32(row.wrapping_add(4));
            let _: u32 =
                lf_checker_rt::callee_cdecl!(SIFT, u32, base, i, n as u32, lo, hi, ctx);
            if i == 0 {
                break;
            }
            i = i.wrapping_sub(1);
        }
        0
    }
});
