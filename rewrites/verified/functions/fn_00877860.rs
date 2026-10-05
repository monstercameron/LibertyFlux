// original: 0x00877860 rage::crmtComposerOptimized::vf2

/// Sync a composer, then tail-forward to the inner object's virtual.
///
/// `this` points to a composer object with a linked flag byte at `+0x11`.
/// When the flag is set the function returns at once (leaving the caller's
/// EAX in place, which no rewrite can reproduce: that path has its own
/// contract with no return channel). Otherwise the pool-sync virtual
/// (slot `+0x78`) and the drain helper run, the free-node cache at
/// `+0x88` is cleared, and control jumps -- not calls -- to the inner
/// object's virtual at slot `+8` (`inner` is the word at `+0x0c`), whose
/// answer becomes this function's return value.
///
/// Original: 0x00877860 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00877860(this: u32) -> u32 {
    unsafe {
        const LINKED_OFF: u32 = 0x11;
        const INNER_OFF: u32 = 0x0c;
        const FREE_OFF: u32 = 0x88;
        const SYNC_VT_SLOT: u32 = 0x78;
        const FWD_VT_SLOT: u32 = 0x08;
        const DRAINER: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if (this.wrapping_add(LINKED_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let vtable = rd32(this);
        let sync: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(rd32(vtable.wrapping_add(SYNC_VT_SLOT)) as usize)
        };
        sync(this);
        lf_checker_rt::callee_thiscall!(DRAINER, u32, this);
        wr32(this.wrapping_add(FREE_OFF), 0);
        let inner = rd32(this.wrapping_add(INNER_OFF));
        let ivtable = rd32(inner);
        let fwd: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(rd32(ivtable.wrapping_add(FWD_VT_SLOT)) as usize)
        };
        fwd(inner)
    }
});
