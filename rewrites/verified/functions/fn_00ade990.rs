// original: 0x00ade990 partial_sort (proposed)

/// Partially sort a range: heap-select the middle part, then sort it.
///
/// `first`, `mid` and `last` bound the array and the selection, the
/// fourth argument is unread, `comp` is a `cdecl(a, b) -> bool`
/// comparator. The make-heap callee (id 2:
/// first, mid, comparator) heaps the selection first. Every element
/// from `mid` to `last` is then tested as `comp(element, *first)`; on
/// success the element trades places with the heap top through the
/// sift-down callee (id 3: first, hole 0, heap word count, evicted
/// value, comparator). Finally the pop callee (id 4: first, end,
/// comparator) is called while shrinking `end` from `mid` down to
/// four bytes past `first`, sorting the selected part.
///
/// Edge cases: an empty tail skips the selection loop; a selection of
/// one word or less skips the pop loop; both loops use unsigned
/// bounds.
///
/// Original: cdecl, five stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments) and three direct callees (cdecl with
/// three, five and three arguments). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade990(first: u32, mid: u32, last: u32, _unused: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        lf_checker_rt::callee_cdecl!(2, u32, first, mid, comp);
        let mut p = mid;
        while p < last {
            if cmp(rd(p), rd(first)) != 0 {
                let old = rd(p);
                wr(p, rd(first));
                let cnt = mid.wrapping_sub(first) >> 2;
                lf_checker_rt::callee_cdecl!(3, u32, first, 0, cnt, old, comp);
            }
            p = p.wrapping_add(4);
        }
        let mut span = mid.wrapping_sub(first);
        let mut end = mid;
        loop {
            let s = span & 0xFFFF_FFFC;
            if (s as i32) <= 4 {
                break;
            }
            lf_checker_rt::callee_cdecl!(4, u32, first, end, comp);
            span = span.wrapping_sub(4);
            end = end.wrapping_sub(4);
        }
    }
    0
});
