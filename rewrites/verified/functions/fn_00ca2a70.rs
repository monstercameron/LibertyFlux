// original: 0x00ca2a70 CEventDamage::CEventDamage_2

/// Construct a damage event from three words, retaining the first.
///
/// Runs the base constructor (callee 1), installs the vtables, stores the
/// three words at `+0x18`/`+0x1c`/`+0x20`, masks the flag bytes at `+0x28`
/// and `+0x38`, zeroes `+0x34`, and retains the first word (callee 2) when
/// it is non-null. Returns `this`.
///
/// Original: 0x00ca2a70 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00ca2a70(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
        const VT_MAIN: u32 = 0x00ed_7424;
        const VT_SUB: u32 = 0x00ed_7418;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        wr32(this + 0x18, a0);
        wr32(this + 0x1c, a1);
        wr32(this + 0x20, a2);
        wr8(this + 0x28, rd8(this + 0x28) & 0xbe);
        wr32(this + 0x24, lf_checker_rt::relocated(VT_SUB));
        wr8(this + 0x38, rd8(this + 0x38) & 0xfc);
        wr32(this + 0x34, 0);
        let h = rd32(this + 0x18);
        if h != 0 {
            lf_checker_rt::callee_thiscall!(2, u32, h, this + 0x18);
        }
        this
    }
});
