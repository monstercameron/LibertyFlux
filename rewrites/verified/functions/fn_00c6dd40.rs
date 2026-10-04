// original: 0x00c6dd40 make_heap_range
/// Turn `[first, last)` into a heap by adjusting every non-leaf slot from
/// the last one down to the root. Ranges shorter than 2 need no work.
export!(cdecl, rw_00c6dd40(first: u32, last: u32, extra: u32) -> () {
    unsafe {
        let count = (last.wrapping_sub(first) as i32) >> 3;
        if count < 2 {
            return;
        }
        let at = |h: i32| first.wrapping_add((h as u32).wrapping_mul(8));
        let mut h = (count - 2) / 2;
        callee_cdecl!(1, u32, first, h as u32, count as u32,
            *(at(h) as *const u32), *(((at(h)) + 4) as *const u32), extra);
        while h != 0 {
            h -= 1;
            callee_cdecl!(1, u32, first, h as u32, count as u32,
                *(at(h) as *const u32), *(((at(h)) + 4) as *const u32), extra);
        }
    }
});
