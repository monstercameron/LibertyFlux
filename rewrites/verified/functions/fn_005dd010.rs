// original: 0x005dd010 CTaskComplexWanderMedic::vf1

/// Clone a wander-medic task from two words, a flag bit and a global rate.
///
/// Allocates through the pool at `POOL`; null stays null. Otherwise
/// constructs the new task from the source words at `+0x14`/`+0x18`,
/// bit 0 of the byte at `+0x54`, the global float at `RATE` and the
/// constant 1; stamps `VTABLE` and returns the new task.
///
/// Original: 0x005dd010 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dd010(this: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0cb4;
        const RATE: u32 = 0xed7ee4;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        let rate = rd32(lf_checker_rt::relocated(RATE));
        let flag = (rd8(this + 0x54) & 1) as u32;
        let w18 = rd32(this + 0x18);
        let w14 = rd32(this + 0x14);
        lf_checker_rt::callee_thiscall!(CTOR, u32, new, w14, w18, flag, rate, 1);
        wr32(new, lf_checker_rt::relocated(VTABLE));
        new
    }
});
