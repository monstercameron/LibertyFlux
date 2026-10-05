// original: 0x005dcea0 CTaskComplexMedicTreatInjuredPed::vf1

/// Clone a medic-treat task, forwarding five fields to the constructor.
///
/// Allocates through the pool at `POOL`; null stays null. Otherwise
/// constructs the new task from the source words at `+0x14`/`+0x18`,
/// the bytes at `+0x1c`/`+0x42` (zero-extended), and a pointer to the
/// vector at `+0x30`; returns the new task.
///
/// Original: 0x005dcea0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dcea0(this: u32) -> u32 {
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
        let w14 = rd32(this + 0x14);
        let w18 = rd32(this + 0x18);
        let b1c = rd8(this + 0x1c) as u32;
        let b42 = rd8(this + 0x42) as u32;
        lf_checker_rt::callee_thiscall!(CTOR, u32, new, w14, w18, b1c, this + 0x30, b42);
        new
    }
});
