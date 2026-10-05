// original: 0x00877890 rage::crmtComposerOptimized::vf6

/// Gate on a check virtual, then drain and forward to the inner object.
///
/// `this` points to a composer object. The check virtual (slot `+0x7c`)
/// runs with (`this`, `a0`); when its low byte is zero the function
/// returns AL 0 at once. Otherwise the drain helper runs, the free-node
/// cache at `+0x88` is cleared, the inner object's virtual at slot `+8`
/// (`inner` is the word at `+0x0c`) runs, the linked flag at `+0x11` and
/// the word at `a0` are cleared, and AL 1 is returned.
///
/// Original: 0x00877890 (thiscall, one stack argument; low byte of the
/// return only, the upper bytes keep the forwarded helper's answer).
lf_checker_rt::export!(thiscall, rw_00877890(this: u32, a0: u32) -> u32 {
    unsafe {
        const LINKED_OFF: u32 = 0x11;
        const INNER_OFF: u32 = 0x0c;
        const FREE_OFF: u32 = 0x88;
        const CHECK_VT_SLOT: u32 = 0x7c;
        const FWD_VT_SLOT: u32 = 0x08;
        const DRAINER: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let vtable = rd32(this);
        let check: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
            core::mem::transmute(rd32(vtable.wrapping_add(CHECK_VT_SLOT)) as usize)
        };
        if (check(this, a0) as u8) == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(DRAINER, u32, this);
        (this.wrapping_add(FREE_OFF) as *mut u32).write_unaligned(0);
        let inner = rd32(this.wrapping_add(INNER_OFF));
        let ivtable = rd32(inner);
        let fwd: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(rd32(ivtable.wrapping_add(FWD_VT_SLOT)) as usize)
        };
        fwd(inner);
        (this.wrapping_add(LINKED_OFF) as *mut u8).write(0);
        (a0 as *mut u32).write_unaligned(0);
        1
    }
});
