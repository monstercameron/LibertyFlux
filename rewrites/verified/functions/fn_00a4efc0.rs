// original: 0x00a4efc0 sort_push_heap (proposed)

/// Sift `val` up the heap toward `top`, parents moving down past it.
///
/// `first` is the element-pointer array, `hole` and `top` are signed
/// indexes. Parents are visited from (hole-1)/2 upward with the original's
/// signed halving; while `val`'s key (float at +0x80) is strictly greater
/// than the parent's, the parent drops into the hole and the hole rises. A
/// key that is less, equal or unordered ends the climb, and `val` lands in
/// the final hole. When `hole` is already at or below `top` only the final
/// store runs. The fifth argument is unused. Cdecl, five stack words, no
/// result.
lf_checker_rt::export!(cdecl, rw_00a4efc0(first: u32, hole: u32, top: u32, val: u32, _extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
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
        /// Signed halving exactly as cdq/sub/sar: (t-sign)>>1.
        fn half_down(t: i32) -> i32 {
            t.wrapping_sub(t >> 31) >> 1
        }
        let top = top as i32;
        let mut hole = hole as i32;
        let mut parent = half_down(hole.wrapping_sub(1));
        if hole > top {
            let want = key(val);
            loop {
                let cand = rd(slot(first, parent));
                if !(want > key(cand)) {
                    break;
                }
                wr(slot(first, hole), cand);
                hole = parent;
                parent = half_down(parent.wrapping_sub(1));
                if !(hole > top) {
                    break;
                }
            }
        }
        wr(slot(first, hole), val);
        0
    }
});
