// original: 0x00ade720 heap_sift_down (proposed)

/// Sift a hole down a binary heap, then sift the value back up.
///
/// `base` is a word array, `hole` the vacant index, `count` the heap
/// size, `val` the value to place, `comp` a `cdecl(a, b) -> bool`
/// comparator. While the second child index `2*hole+2` is below
/// `count`, the two children are compared as `comp(right, left)` and
/// the hole moves to the loser-or-first child, pulling that child's
/// value up. When the loop ends exactly at `count`, the last value is
/// pulled up and the hole moves below it. The push-up callee (id 2,
/// the sift-up function) then finishes the placement from the final
/// hole down to the original one.
///
/// Edge cases: an empty range stores nothing before the callee; a hole
/// already at or past the end skips the loop and goes straight to the
/// callee.
///
/// Original: cdecl, five stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments) and one direct callee (id 2, cdecl,
/// five arguments: base, final hole, original hole, value, comparator).
/// Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade720(base: u32, hole: u32, count: u32, val: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        let mut h = hole;
        let mut child = hole.wrapping_mul(2).wrapping_add(2);
        if child < count {
            loop {
                let left = rd(base.wrapping_add(child.wrapping_mul(4)).wrapping_sub(4));
                let right = rd(base.wrapping_add(child.wrapping_mul(4)));
                if cmp(right, left) != 0 {
                    child = child.wrapping_sub(1);
                }
                wr(base.wrapping_add(h.wrapping_mul(4)), rd(base.wrapping_add(child.wrapping_mul(4))));
                h = child;
                child = child.wrapping_mul(2).wrapping_add(2);
                if child >= count {
                    break;
                }
            }
        }
        if child == count {
            wr(
                base.wrapping_add(h.wrapping_mul(4)),
                rd(base.wrapping_add(child.wrapping_mul(4)).wrapping_sub(4)),
            );
            h = child.wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(2, u32, base, h, hole, val, comp);
    }
    0
});
