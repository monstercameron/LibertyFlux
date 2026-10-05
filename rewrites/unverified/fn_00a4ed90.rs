// original: 0x00a4ed90 sort_insertion_unguarded (proposed)

/// Insertion sort over [first, last): front-insert or linear-insert each.
///
/// For each slot past the first, its key is compared against the first
/// slot's key: a strictly smaller key goes to the front (the whole sorted
/// prefix shifts right one slot through the move callee, then the element
/// lands at the front); anything else (greater, equal or unordered) is
/// placed by the linear-insert callee, which is safe unguarded because the
/// front element stops its scan. The third argument is unused, the fourth
/// (`extra`) is threaded to the linear-insert callee. Cdecl, four stack
/// words, two callees, no result.
lf_checker_rt::export!(cdecl, rw_00a4ed90(first: u32, last: u32, _u: u32, extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        const MOVE: u32 = 1;
        const INSERT: u32 = 2;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        if first == last {
            return 0;
        }
        let mut cur = first.wrapping_add(4);
        if cur == last {
            return 0;
        }
        loop {
            let a = rd(first);
            let b = rd(cur);
            if key(a) > key(b) {
                let n = cur.wrapping_sub(first);
                if (n as i32) > 0 {
                    let dst = cur.wrapping_sub(n).wrapping_add(4);
                    lf_checker_rt::callee_cdecl!(MOVE, u32, dst, first, n);
                }
                wr(first, b);
            } else {
                lf_checker_rt::callee_cdecl!(INSERT, u32, cur, b, extra);
            }
            cur = cur.wrapping_add(4);
            if cur == last {
                break;
            }
        }
        0
    }
});
