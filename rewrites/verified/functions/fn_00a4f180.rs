// original: 0x00a4f180 sort_std_sort (proposed)

/// Top-level sort: introsort loop then insertion sort over [first, last).
///
/// Empty ranges return at once. Otherwise the depth limit is twice the
/// number of halvings that reduce the element count to one (zero for a
/// single element), the introsort callee runs with that limit, and the
/// insertion-sort callee finishes the ranges the loop leaves small. `extra`
/// is threaded to both. Cdecl, three stack words, two callees, no result.
lf_checker_rt::export!(cdecl, rw_00a4f180(first: u32, last: u32, extra: u32) -> u32 {
    unsafe {
        const INTRO: u32 = 1;
        const INSERTION: u32 = 2;
        if first == last {
            return 0;
        }
        let n = ((last.wrapping_sub(first)) as i32) >> 2;
        let mut depth: u32 = 0;
        if n != 1 {
            let mut e = n;
            loop {
                e >>= 1;
                depth = depth.wrapping_add(1);
                if e == 1 {
                    break;
                }
            }
        }
        lf_checker_rt::callee_cdecl!(INTRO, u32, first, last, 0, depth.wrapping_mul(2), extra);
        lf_checker_rt::callee_cdecl!(INSERTION, u32, first, last, extra);
        0
    }
});
