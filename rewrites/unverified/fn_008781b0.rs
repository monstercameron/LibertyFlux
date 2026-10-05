// original: 0x008781B0 rage::crmtComposerOptimized::vf20

/// Advance a composer's sequence counter, refilling the node pool first.
///
/// `this` points to a composer object with a 16-bit sequence counter at
/// `+0x72` and an immediate-mode flag byte at `+0x98`. Unless immediate
/// mode is on, the pool-refill virtual (slot `+0x78`) runs first; then the
/// counter grows by 2. Takes two stack words, neither read.
///
/// Original: 0x008781B0 (thiscall, two stack arguments). No return value.
lf_checker_rt::export!(thiscall, rw_008781B0(this: u32, _a0: u32, _a1: u32) -> u32 {
    unsafe {
        const SEQ_OFF: u32 = 0x72;
        const IMMED_OFF: u32 = 0x98;
        const SEQ_STEP: u16 = 2;
        const REFILL_VT_SLOT: u32 = 0x78;
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        if rd8(this.wrapping_add(IMMED_OFF)) == 0 {
            let vtable = (this as *const u32).read_unaligned();
            let refill: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(
                    ((vtable.wrapping_add(REFILL_VT_SLOT)) as *const u32).read_unaligned()
                        as usize,
                )
            };
            refill(this);
        }
        let seq = rd16(this.wrapping_add(SEQ_OFF));
        wr16(this.wrapping_add(SEQ_OFF), seq.wrapping_add(SEQ_STEP));
        0
    }
});
