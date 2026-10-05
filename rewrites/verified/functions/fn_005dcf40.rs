// original: 0x005dcf40 CTaskComplexMedicWandering::vf1

/// Clone a medic-wandering task via a tail call to its constructor.
///
/// `this` is unused. Allocates through the pool at `POOL`; null stays
/// null. Otherwise the original tail-jumps to the constructor with the
/// new task; the rewrite calls it and returns the new task.
///
/// Original: 0x005dcf40 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dcf40(this: u32) -> u32 {
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
        // The original tail-jumps to the constructor; a call returning the
        // new task observes the same calls and result.
        lf_checker_rt::callee_thiscall!(CTOR, u32, new);
        new
    }
});
