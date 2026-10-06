// original: 0x00693CA0 comp_remove_by_id (proposed)

//
// Removes every element whose dword at +4 equals `id` from the array at
// [obj] (count as a 16-bit word at [obj+4]): each match is released
// through the hook at its vtable slot +0 with argument 1, later slots
// shift down one, and the count drops. A removed slot is re-examined, so
// adjacent matches all go. Returns 1 in the low byte when anything was
// removed, else 0. (The original reads [elem+4] before its null test, so
// a null element faults on both sides; the corpus keeps elements live.)
//
// Original: 0x00693CA0 (thiscall, one stack argument, callee-cleanup,
// byte return).
lf_checker_rt::export!(thiscall, rw_00693CA0(obj: u32, id: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const RELEASE_SLOT: u32 = 0x00;
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let mut found = 0u32;
        let mut i = 0u32;
        if 0u32 >= rd16(obj.wrapping_add(4)) as u32 {
            return 0;
        }
        loop {
            let count = rd16(obj.wrapping_add(4)) as u32;
            if i >= count {
                break;
            }
            let base = rd32(obj);
            let elem = rd32(base.wrapping_add(i.wrapping_mul(4)));
            if rd32(elem.wrapping_add(4)) == id {
                let vtable = rd32(elem);
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(RELEASE_SLOT)) as usize);
                release(elem, 1);
                let mut j = i;
                while (j as i32) < ((count as i32).wrapping_sub(1)) {
                    let b2 = rd32(obj);
                    let nxt = rd32(b2.wrapping_add(j.wrapping_add(1).wrapping_mul(4)));
                    wr32(b2.wrapping_add(j.wrapping_mul(4)), nxt);
                    j = j.wrapping_add(1);
                }
                wr16(obj.wrapping_add(4), count.wrapping_sub(1) as u16);
                found = 1;
            } else {
                i = i.wrapping_add(1);
            }
        }
        found
    }
});
