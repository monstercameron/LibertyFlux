// original: 0x00ca2a40 CEventDamage::CEventDamage

/// Construct a damage event around one argument word.
///
/// Runs the base constructor (callee 1), installs the two vtable pointers,
/// clears flag bits 0, 5 and 6 of the byte at `+0x28`, then runs the
/// payload constructor (callee 2) with the argument word. Returns `this`.
///
/// Original: 0x00ca2a40 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ca2a40(this: u32, arg: u32) -> u32 {
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
        const KEEP_MASK: u8 = 0xbe;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        wr8(this + 0x28, rd8(this + 0x28) & KEEP_MASK);
        wr32(this + 0x24, lf_checker_rt::relocated(VT_SUB));
        lf_checker_rt::callee_thiscall!(2, u32, this, arg);
        this
    }
});
