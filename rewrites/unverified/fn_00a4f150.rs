// original: 0x00a4f150 sort_pop_heap (proposed)

/// Pop the heap top: swap the first and last elements, re-heapify the rest.
///
/// The elements at `first` and `last-4` are exchanged (the last slot keeps
/// the old top), then the adjust-heap callee repairs [first, last-4) from
/// hole zero with the displaced last element and `extra`. Cdecl, three
/// stack words, one callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4f150(first: u32, last: u32, extra: u32) -> u32 {
    unsafe {
        const ADJUST: u32 = 1;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let a = rd(first);
        let c = rd(last.wrapping_sub(4));
        wr(last.wrapping_sub(4), a);
        let n = ((last.wrapping_sub(first).wrapping_sub(4)) as i32) >> 2;
        lf_checker_rt::callee_cdecl!(ADJUST, u32, first, 0, n as u32, c, extra);
        0
    }
});
