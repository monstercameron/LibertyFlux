// original: 0x006933A0 track_heap_build (proposed)

/// Builds a heap out of the array at `base`: `end` points one past the
/// last word, so (end-base)/4 is the slot count. From slot (count-2)/2
/// (truncating division) down to 0, each slot's element is lifted out, the
/// hole is sifted down exactly like track_heap_sift_down (larger child
/// wins, unsigned keys, ties keep the right child; signed bound of count),
/// and the intercepted place helper is called with (base, hole, slot,
/// lifted, flag) where flag is the byte at `arg`. No return value.
///
/// Original: 0x006933A0 (ECX+EDX plus one stack argument with caller
/// cleanup, so the esp check is off for this function).
lf_checker_rt::export!(fastcall, rw_006933A0(base: u32, end: u32, arg: u32) -> u32 {
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

        let count = ((end.wrapping_sub(base) as i32) >> 2) as u32;
        let mut b = ((count as i32).wrapping_sub(2) / 2) as u32;
        let mut child0 = b.wrapping_mul(2).wrapping_add(2);
        loop {
            let saved = rd32(base.wrapping_add(b.wrapping_mul(4)));
            let flag = rd8(arg);
            let mut eax = child0;
            let mut slot = b;
            if (eax as i32) < (count as i32) {
                loop {
                    let right = rd32(base.wrapping_add(eax.wrapping_mul(4)));
                    let left = rd32(base.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(4));
                    if track_key(right) < track_key(left) {
                        eax = eax.wrapping_sub(1);
                    }
                    let moving = rd32(base.wrapping_add(eax.wrapping_mul(4)));
                    wr32(base.wrapping_add(slot.wrapping_mul(4)), moving);
                    slot = eax;
                    eax = eax.wrapping_mul(2).wrapping_add(2);
                    if !((eax as i32) < (count as i32)) {
                        break;
                    }
                }
            }
            if eax == count {
                let last = rd32(base.wrapping_add(eax.wrapping_mul(4)).wrapping_sub(4));
                wr32(base.wrapping_add(slot.wrapping_mul(4)), last);
                slot = eax.wrapping_sub(1);
            }
            lf_checker_rt::callee_fastcall!(PLACE, u32, base, slot, b, saved, flag as u32);
            if b == 0 {
                break;
            }
            b = b.wrapping_sub(1);
            child0 = child0.wrapping_sub(2);
        }
        0
    }
});
