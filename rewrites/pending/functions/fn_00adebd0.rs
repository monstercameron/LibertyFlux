// original: 0x00adebd0 std_sort (proposed)

/// Sort a word range: introsort followed by insertion sort.
///
/// `first` and `last` bound the array, `comp` is the comparator passed
/// through to both callees. An empty range returns at once. Otherwise
/// the depth allowance is twice the floor of the base-2 logarithm of
/// the element count, the introsort callee (id 1: first, last, 0,
/// depth, comparator) runs, and the insertion-dispatch callee (id 2:
/// first, last, comparator) finishes the nearly sorted range.
///
/// Edge cases: empty ranges make no calls; a single element runs both
/// callees with a zero depth allowance.
///
/// Original: cdecl, three stack words, two direct callees (ids 2 and 3,
/// cdecl with five and three arguments; id 1 only supplies the
/// comparator stub address passed through to them). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00adebd0(first: u32, last: u32, comp: u32) -> u32 {
    if first == last {
        return 0;
    }
    let mut n = last.wrapping_sub(first) >> 2;
    let mut depth: u32 = 0;
    if n != 1 {
        loop {
            n >>= 1;
            depth = depth.wrapping_add(1);
            if n == 1 {
                break;
            }
        }
    }
    lf_checker_rt::callee_cdecl!(2, u32, first, last, 0, depth.wrapping_add(depth), comp);
    lf_checker_rt::callee_cdecl!(3, u32, first, last, comp);
    0
});
