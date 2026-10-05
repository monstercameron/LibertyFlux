// original: 0x008782C0 rage::crmtComposerOptimized::vf25

/// Advance a composer's sequence counter and read back slot `+0x18`.
///
/// `this` points to a composer object with a 16-bit sequence counter at
/// `+0x72` and an immediate-mode flag byte at `+0x98`. Unless immediate
/// mode is on, the pool-refill virtual (slot `+0x78`) runs first; then the
/// counter grows by 2 and the word at `+0x18` is returned. Takes two stack
/// words, neither read.
///
/// Original: 0x008782C0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_008782C0(this: u32, _a0: u32, _a1: u32) -> u32 {
    unsafe {
        const SEQ_OFF: u32 = 0x72;
        const OUT_OFF: u32 = 0x18;
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
        let seq = rd16(this.wrapping_add(SEQ_OFF));
        wr16(this.wrapping_add(SEQ_OFF), seq.wrapping_add(SEQ_STEP));
        (this.wrapping_add(OUT_OFF) as *const u32).read_unaligned()
    }
});
