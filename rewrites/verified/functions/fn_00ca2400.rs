// original: 0x00ca2400 swap_ref_148

/// Swap the link at `+0x148`, adjusting refcounts through the vtable.
///
/// The old link, when non-null, goes through vtable slot 0x54 (callee 1)
/// and the byte at `+0x15` of the answer is decremented; the new value is
/// stored, and when non-null goes through the same slot with the answer
/// byte incremented. No value is returned. Both indirect calls use the
/// fabricated objects exactly like the original.
///
/// Original: 0x00ca2400 (thiscall, one stack word = new link).
lf_checker_rt::export!(thiscall, rw_00ca2400(this: u32, new: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const LINK: u32 = 0x148;
        const SLOT: u32 = 0x54;
        const REFCOUNT: u32 = 0x15;
        let old = rd32(this + LINK);
        if old != 0 {
            let slot = rd32(rd32(old) + SLOT);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let back = f(old);
            wr8(back + REFCOUNT, rd8(back + REFCOUNT).wrapping_sub(1));
        }
        wr32(this + LINK, new);
        if new != 0 {
            let slot = rd32(rd32(new) + SLOT);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(slot as usize);
            let back = f(new);
            wr8(back + REFCOUNT, rd8(back + REFCOUNT).wrapping_add(1));
        }
        0
    }
});
