// original: 0x00ca2b30 CEventDeath::CEventDeath_2

/// Construct a death event with an explicit stamp word.
///
/// Runs the base constructor (callee 1), stores the low byte of the first
/// word at `+0x0c`, the second word at `+0x10`, and installs the vtable.
/// Returns `this`.
///
/// Original: 0x00ca2b30 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ca2b30(this: u32, a0: u32, a1: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const VT_MAIN: u32 = 0x00ed_7484;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        wr8(this + 0x0c, (a0 & 0xff) as u8);
        wr32(this + 0x10, a1);
        wr32(this, lf_checker_rt::relocated(VT_MAIN));
        this
    }
});
