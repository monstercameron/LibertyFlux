// original: 0x00693510 track_heap_sift_up (proposed)

/// Sifts `elem` up a binary heap of track pointers: `base` is the array,
/// `idx` the starting slot, `lo` the lowest slot to fill. The parent of
/// slot n is (n-1)/2 with truncation toward zero (the original computes it
/// with cdq/sub/sar, which matches truncating division for every input).
/// While the slot is strictly above `lo` (signed comparison) and the
/// parent's key is strictly below the element's key (unsigned comparison
/// of the 24-bit track keys), the parent moves down and the slot rises to
/// the parent; the element lands in the final slot. No return value.
///
/// Original: 0x00693510 (ECX+EDX plus two stack arguments with caller
/// cleanup, which has no Rust equivalent, so the esp check is off).
lf_checker_rt::export!(fastcall, rw_00693510(base: u32, idx: u32, lo: u32, elem: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn track_key(t: u32) -> u32 {
            unsafe { ((rd8(t.wrapping_add(5)) as u32) << 16) | (rd16(t.wrapping_add(6)) as u32) }
        }

        let want = track_key(elem);
        let mut slot = idx;
        if (slot as i32) > (lo as i32) {
            let mut parent = ((slot as i32).wrapping_sub(1) / 2) as u32;
            loop {
                let pe = rd32(base.wrapping_add(parent.wrapping_mul(4)));
                if track_key(pe) >= want {
                    break;
                }
                wr32(base.wrapping_add(slot.wrapping_mul(4)), pe);
                slot = parent;
                parent = ((slot as i32).wrapping_sub(1) / 2) as u32;
                if (slot as i32) <= (lo as i32) {
                    break;
                }
            }
        }
        wr32(base.wrapping_add(slot.wrapping_mul(4)), elem);
        0
    }
});
