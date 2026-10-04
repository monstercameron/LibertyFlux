// original: 0x00abbd40 sift_down_then_up

/// Sift a hole down a sentinel-terminated binary heap, then sift it back up.
///
/// Starting from index `i`, repeatedly moves one child into the hole: the
/// right child when it holds the empty-slot marker, otherwise the left
/// child. When the hole reaches the end exactly, the last element closes it.
/// Afterwards the companion sift-up routine (stubbed by the checker) is
/// invoked with the final hole position, and its answer is returned.
export!(cdecl, rs64_abbd40(base: *const u32, i: u32, n: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const EMPTY_FILEVA: u32 = 0x0151_0A90;
        let empty = relocated(EMPTY_FILEVA);
        let mut hole = i;
        let mut c = i.wrapping_mul(2).wrapping_add(2);
        if (c as i32) < (n as i32) {
            loop {
                if *base.add(c as usize) != empty {
                    c = c.wrapping_sub(1);
                }
                let v = *base.add(c as usize);
                *(base.add(hole as usize) as *mut u32) = v;
                hole = c;
                c = c.wrapping_mul(2).wrapping_add(2);
                if !((c as i32) < (n as i32)) {
                    break;
                }
            }
        }
        if c == n {
            let v = *base.add((c as usize).wrapping_sub(1));
            *(base.add(hole as usize) as *mut u32) = v;
            hole = c.wrapping_sub(1);
        }
        callee_cdecl!(0, u32, base as u32, hole, i, a3, a4)
    }
});
