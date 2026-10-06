// original: 0x008E65F0 introsort_range (proposed)

/// Sort one range: directly when small, split at 0x80 bytes when large.
///
/// `n` is the range length `(hi - lo)` rounded down to 8 bytes. When `n`
/// is at most `SPLIT` (signed) the small-range sorter (callee 1, cdecl) is
/// invoked once as `(lo, hi, 0, ctx)`. Otherwise the range is split at
/// `lo + SPLIT`: the small-range sorter handles `(lo, mid, 0, ctx)` and the
/// large-range sorter (callee 2, cdecl) handles `(mid, hi, 0, ctx)`.
/// Returns the last callee's answer.
///
/// Original: 0x008E65F0 (cdecl, three stack arguments).
lf_checker_rt::export!(cdecl, rw_008E65F0(lo: u32, hi: u32, ctx: u32) -> u32 {
    unsafe {
        /// Split point: ranges longer than this are divided.
        const SPLIT: u32 = 0x80;
        /// Length mask: range lengths round down to 8 bytes.
        const LEN_MASK: u32 = 0xFFFF_FFF8;
        /// Small-range sorter callee id.
        const SMALL: u32 = 1;
        /// Large-range sorter callee id.
        const LARGE: u32 = 2;

        let n = hi.wrapping_sub(lo) & LEN_MASK;
        if (n as i32) <= (SPLIT as i32) {
            lf_checker_rt::callee_cdecl!(SMALL, u32, lo, hi, 0, ctx)
        } else {
            let mid = lo.wrapping_add(SPLIT);
            let _: u32 = lf_checker_rt::callee_cdecl!(SMALL, u32, lo, mid, 0, ctx);
            lf_checker_rt::callee_cdecl!(LARGE, u32, mid, hi, 0, ctx)
        }
    }
});
