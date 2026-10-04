// original: 0x00c6db30 adjust_heap_hole
/// Sift the hole at `hole` down through the heap of `count` elements at
/// `base` (moving the smaller-signed-key child up each step), then insert
/// (key, value) at the final hole through the push-up helper. The sixth
/// argument is forwarded untouched. Returns the helper's answer.
export!(cdecl, rw_00c6db30(base: u32, hole: u32, count: u32, key: u32, val: u32, extra: u32) -> u32 {
    unsafe {
        let w = base as *mut u32;
        let n = count as i32;
        let mut h = hole as i32;
        let mut c = h.wrapping_mul(2).wrapping_add(2);
        if c < n {
            loop {
                let prev = *(w.add(((c - 1) as usize) * 2)) as i32;
                let cur = *(w.add((c as usize) * 2)) as i32;
                if cur < prev {
                    c -= 1;
                }
                *w.add((h as usize) * 2) = *w.add((c as usize) * 2);
                *w.add((h as usize) * 2 + 1) = *w.add((c as usize) * 2 + 1);
                h = c;
                c = c.wrapping_mul(2).wrapping_add(2);
                if c >= n {
                    break;
                }
            }
        }
        if c == n {
            *w.add((h as usize) * 2) = *w.add(((n - 1) as usize) * 2);
            *w.add((h as usize) * 2 + 1) = *w.add(((n - 1) as usize) * 2 + 1);
            h = n - 1;
        }
        callee_cdecl!(1, u32, base, h as u32, hole, key, val, extra)
    }
});
