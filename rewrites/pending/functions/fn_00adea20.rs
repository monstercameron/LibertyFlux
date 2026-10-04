// original: 0x00adea20 push_heap_sift_up (proposed)

/// Sift a value up a binary heap from a hole toward the top.
///
/// `base` is a word array, `hole` the vacant index, `top` the highest
/// index the hole may rise past, `val` the value to place, `comp` a
/// `cdecl(a, b) -> bool` comparator. When `hole` is already at or
/// below `top` the value is stored directly. Otherwise, while the hole
/// stays above `top`, the parent at `(hole-1)/2` is tested as
/// `comp(parent_value, val)`; a false answer ends the walk, a true one
/// pulls the parent down and continues from the parent. The value is
/// stored at the final hole.
///
/// Edge cases: a hole of zero stores directly; the parent division is
/// the signed sequence from the original, exact for all inputs.
///
/// Original: cdecl, five stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments), no direct callees. Returns nothing.
lf_checker_rt::export!(cdecl, rw_00adea20(base: u32, hole: u32, top: u32, val: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        // Exact copy of the original's (h-1)/2: subtract with borrow
        // folded in, then arithmetic shift.
        let parent = |h: u32| {
            let eax = h.wrapping_sub(1);
            let edx: u32 = if eax & 0x8000_0000 != 0 { 0xFFFF_FFFF } else { 0 };
            ((eax.wrapping_sub(edx) as i32) >> 1) as u32
        };
        let mut h = hole;
        let mut p = parent(hole);
        if (hole as i32) > (top as i32) {
            loop {
                if cmp(rd(base.wrapping_add(p.wrapping_mul(4))), val) == 0 {
                    break;
                }
                wr(base.wrapping_add(h.wrapping_mul(4)), rd(base.wrapping_add(p.wrapping_mul(4))));
                h = p;
                p = parent(p);
                if !((h as i32) > (top as i32)) {
                    break;
                }
            }
        }
        wr(base.wrapping_add(h.wrapping_mul(4)), val);
    }
    0
});
