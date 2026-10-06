// original: 0x00693480 track_heap_sift_down (proposed)

/// Sifts the hole at slot `idx` down a binary heap of track pointers with
/// `bound` slots (signed bound): while slot 2*n+2 is strictly below the
/// bound (signed), the larger of the two children (unsigned key compare;
/// ties keep the right child) moves into the hole and the hole follows it.
/// When the loop ends exactly at the bound, the single last child moves
/// into the hole. Then the intercepted place helper is called with
/// (base, hole, idx, elem, extra) to settle `elem`: note the third word is
/// the entry idx, reloaded from a saved slot (the bound lives in ECX and is
/// never pushed). No return value.
///
/// Original: 0x00693480 (ECX+EDX plus three stack arguments with caller
/// cleanup, so the esp check is off for this function).
lf_checker_rt::export!(fastcall, rw_00693480(base: u32, idx: u32, bound: u32, elem: u32, extra: u32) -> u32 {
    unsafe {
        const PLACE: u32 = 1;

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

        let mut slot = idx;
        let mut child = idx.wrapping_mul(2).wrapping_add(2);
        if (child as i32) < (bound as i32) {
            loop {
                let right = rd32(base.wrapping_add(child.wrapping_mul(4)));
                let left = rd32(base.wrapping_add(child.wrapping_mul(4)).wrapping_sub(4));
                if track_key(right) < track_key(left) {
                    child = child.wrapping_sub(1);
                }
                let moving = rd32(base.wrapping_add(child.wrapping_mul(4)));
                wr32(base.wrapping_add(slot.wrapping_mul(4)), moving);
                slot = child;
                child = child.wrapping_mul(2).wrapping_add(2);
                if !((child as i32) < (bound as i32)) {
                    break;
                }
            }
        }
        if child == bound {
            let last = rd32(base.wrapping_add(child.wrapping_mul(4)).wrapping_sub(4));
            wr32(base.wrapping_add(slot.wrapping_mul(4)), last);
            slot = child.wrapping_sub(1);
        }
        lf_checker_rt::callee_fastcall!(PLACE, u32, base, slot, idx, elem, extra);
        0
    }
});
