// original: 0x00ade930 make_heap (proposed)

/// Turn a word range into a heap by sifting every parent down.
///
/// `first` and `last` bound the array, `comp` is the comparator passed
/// through to the sift-down callee (id 1: base, hole, count, value,
/// comparator). Ranges shorter than two words return at once.
/// Otherwise the sift starts at index `(count-2)/2` and walks down to
/// zero, each step sifting the element at that index into the heap of
/// `count` elements.
///
/// Edge cases: empty and single-element ranges make no calls; index
/// zero is always sifted last.
///
/// Original: cdecl, three stack words, one direct callee (id 2, cdecl,
/// five arguments; id 1 only supplies the comparator stub address
/// passed through to it). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade930(first: u32, last: u32, comp: u32) -> u32 {
    unsafe {
        let n = (last.wrapping_sub(first) as i32) >> 2;
        if n < 2 {
            return 0;
        }
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let mut i = (n - 2) / 2;
        lf_checker_rt::callee_cdecl!(
            2, u32, first, i as u32, n as u32,
            rd(first.wrapping_add((i as u32).wrapping_mul(4))), comp
        );
        while i != 0 {
            i -= 1;
            lf_checker_rt::callee_cdecl!(
                2, u32, first, i as u32, n as u32,
                rd(first.wrapping_add((i as u32).wrapping_mul(4))), comp
            );
        }
    }
    0
});
