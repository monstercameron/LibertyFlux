// original: 0x00a4ecd0 sort_adjust_heap (proposed)

/// Sift the hole down the heap, then push `val` up from where it lands.
///
/// `first` is the element-pointer array; `hole` and `last` are signed
/// bounds. The children of each hole (2h+1, 2h+2) are compared by key and
/// the larger child moves up (ties and NaN keep the right child); a lone
/// last child moves up when the child index exactly reaches `last`. The
/// sifted hole, the starting hole as top, and `val` go to the push-heap
/// callee with `extra` threaded through. Cdecl, five stack words, one
/// callee, no result.
lf_checker_rt::export!(cdecl, rw_00a4ecd0(first: u32, hole: u32, last: u32, val: u32, extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        const PUSH_HEAP: u32 = 1;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        fn slot(first: u32, idx: i32) -> u32 {
            first.wrapping_add((idx.wrapping_mul(4)) as u32)
        }
        let last = last as i32;
        let top = hole as i32;
        let mut h = top;
        let mut d = h.wrapping_mul(2).wrapping_add(2);
        if d < last {
            loop {
                let left = rd(slot(first, d.wrapping_sub(1)));
                let right = rd(slot(first, d));
                if key(left) > key(right) {
                    d = d.wrapping_sub(1);
                }
                wr(slot(first, h), rd(slot(first, d)));
                h = d;
                d = d.wrapping_mul(2).wrapping_add(2);
                if !(d < last) {
                    break;
                }
            }
        }
        if d == last {
            wr(slot(first, h), rd(slot(first, d.wrapping_sub(1))));
            h = d.wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(PUSH_HEAP, u32, first, h as u32, top as u32, val, extra);
        0
    }
});
