// original: 0x00878250 rage::crmtComposerOptimized::vf23

/// Peek at (or pop) the current item of a composer's inner stack.
///
/// `this` points to a composer object with an immediate-mode flag byte at
/// `+0x98` and an inner stack object at `+0x0c` (item table at its `+4`,
/// one-past-top index at its `+0x14`). Unless immediate mode is on, the
/// pool-refill virtual (slot `+0x78`) runs first. Then: when the low byte
/// of `mode` is non-zero the top item is only read (peek); when it is
/// zero the index is decremented first (pop). Either way the item that was
/// on top is returned.
///
/// Original: 0x00878250 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00878250(this: u32, mode: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x0c;
        const TABLE_OFF: u32 = 0x04;
        const TOP_OFF: u32 = 0x14;
        const IMMED_OFF: u32 = 0x98;
        const REFILL_VT_SLOT: u32 = 0x78;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if (this.wrapping_add(IMMED_OFF) as *const u8).read() == 0 {
            let vtable = (this as *const u32).read_unaligned();
            let refill: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((vtable.wrapping_add(REFILL_VT_SLOT)) as *const u32).read_unaligned()
                        as usize,
                )
            };
            refill(this);
        }
        let inner = rd32(this.wrapping_add(INNER_OFF));
        let top = rd32(inner.wrapping_add(TOP_OFF));
        if (mode as u8) == 0 {
            wr32(inner.wrapping_add(TOP_OFF), top.wrapping_sub(1));
        }
        let table = rd32(inner.wrapping_add(TABLE_OFF));
        rd32(table.wrapping_add(top.wrapping_mul(4)))
    }
});
