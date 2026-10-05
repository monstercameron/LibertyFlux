// original: 0x00a4eed0 sort_make_heap (proposed)

/// Heapify [first, last): adjust every parent from the middle down to zero.
///
/// The count is (last-first)/4 (signed arithmetic shift); ranges under two
/// elements need no work. The adjust-heap callee runs for index (count-2)/2
/// down to 0, each call receiving the element currently at its index plus
/// `extra`. The last two arguments are unused. Cdecl, five stack words, one
/// callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4eed0(first: u32, last: u32, extra: u32, _u1: u32, _u2: u32) -> u32 {
    unsafe {
        const ADJUST: u32 = 1;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        fn slot(first: u32, idx: i32) -> u32 {
            first.wrapping_add((idx.wrapping_mul(4)) as u32)
        }
        let count = ((last.wrapping_sub(first)) as i32) >> 2;
        if count < 2 {
            return 0;
        }
        let mut h = (count.wrapping_sub(2)) >> 1;
        loop {
            let v = rd(slot(first, h));
            lf_checker_rt::callee_cdecl!(ADJUST, u32, first, h as u32, count as u32, v, extra);
            if h == 0 {
                break;
            }
            h = h.wrapping_sub(1);
        }
        0
    }
});
