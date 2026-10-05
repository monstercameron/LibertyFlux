// original: 0x005dce20 CTaskSimpleAssessInjuredPed::vf1

/// Clone an assess-injured-ped task, forwarding field +0x1c to the constructor.
///
/// Allocates through the pool at `POOL`; null stays null. Otherwise
/// constructs the new task with the source's word at `+0x1c` and
/// returns the new task (the constructor returns its own object).
///
/// Original: 0x005dce20 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dce20(this: u32) -> u32 {
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
        const POOL: u32 = 0x167e2a0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        let arg = rd32(this + 0x1c);
        lf_checker_rt::callee_thiscall!(CTOR, u32, new, arg);
        new
    }
});
