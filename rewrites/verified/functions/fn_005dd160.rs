// original: 0x005dd160 CTaskSimpleAssessInjuredPed_dtor

/// Destroy an assess-injured-ped task, releasing two members.
///
/// Stamps `VTABLE`, releases the member at `+0x1c` when non-null
/// (clearing the slot), tears down the member at `+0x20` with two
/// calls when non-null (clearing it), destroys the member at `+0x34`,
/// then tail-calls the base destructor and returns its result.
///
/// Original: 0x005dd160 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dd160(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const VTABLE: u32 = 0xfe0c5c;
        const NEG1000: u32 = 0xc47a0000;
        const RELEASE: u32 = 1;
        const UNK_A: u32 = 2;
        const UNK_B: u32 = 3;
        const SUB_DTOR: u32 = 4;
        const TAIL_DTOR: u32 = 5;
        wr32(this, lf_checker_rt::relocated(VTABLE));
        let c = rd32(this + 0x1c);
        if c != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, c, this + 0x1c);
            wr32(this + 0x1c, 0);
        }
        let d = rd32(this + 0x20);
        if d != 0 {
            lf_checker_rt::callee_thiscall!(UNK_A, u32, d, this);
            lf_checker_rt::callee_thiscall!(UNK_B, u32, d, NEG1000);
            wr32(this + 0x20, 0);
        }
        lf_checker_rt::callee_thiscall!(SUB_DTOR, u32, this + 0x34);
        // The original tail-jumps to the base destructor; its result is ours.
        lf_checker_rt::callee_thiscall!(TAIL_DTOR, u32, this)
    }
});
