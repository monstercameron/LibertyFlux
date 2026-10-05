// original: 0x0093e270 heap_sort_depth (proposed)

/// Sort the range [`begin`, `end`) with a depth limit from its length.
///
/// An empty range makes no calls. Otherwise derives a depth of twice the
/// bit length of count - 1 (0 for a single element: the shift loop is
/// skipped when count is 1), calls the first callee with (`begin`, `end`,
/// 0, depth, `extra`), then the second with (`begin`, `end`, `extra`).
/// The return value is the last callee answer, or entry garbage for an
/// empty range, so it is not compared.
///
/// Original: 0x0093e270 (cdecl, three stack words; two direct callees).
lf_checker_rt::export!(cdecl, rw_0093e270(begin: u32, end: u32, extra: u32) -> u32 {
    const SORT_A: u32 = 1;
    const SORT_B: u32 = 2;
    unsafe {
        if begin == end {
            return 0;
        }
        // Signed throughout, as the original's arithmetic shifts: a negative
        // length never shifts down to 1, so the original does not terminate
        // there either (the proof keeps end >= begin).
        let mut n = (end.wrapping_sub(begin) as i32) >> 2;
        let mut depth: u32 = 0;
        if n != 1 {
            loop {
                n >>= 1;
                depth += 1;
                if n == 1 {
                    break;
                }
            }
        }
        lf_checker_rt::callee_cdecl!(SORT_A, u32, begin, end, 0u32, depth.wrapping_mul(2), extra);
        lf_checker_rt::callee_cdecl!(SORT_B, u32, begin, end, extra);
        0
    }
});
