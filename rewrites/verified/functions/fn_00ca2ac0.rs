// original: 0x00ca2ac0 CEventDeadPed::CEventDeadPed

/// Construct a dead-ped event, retaining its subject handle.
///
/// Runs the base constructor (callee 1), installs the vtable, stores the
/// subject handle at `+0x18`, the low byte of the second word at `+0x1c`
/// and the third word at `+0x20`, and retains the handle (callee 2) when
/// non-null. Returns `this`.
///
/// Original: 0x00ca2ac0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00ca2ac0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const VT_MAIN: u32 = 0x00ed_74dc;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        wr8(this + 0x1c, (a1 & 0xff) as u8);
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        wr32(this + 0x18, a0);
        wr32(this + 0x20, a2);
        let h = rd32(this + 0x18);
        if h != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, h, this + 0x18);
        }
        this
    }
});
