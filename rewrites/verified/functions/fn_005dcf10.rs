// original: 0x005dcf10 CTaskComplexMedicPassenger::vf1

/// Clone a medic-passenger task, forwarding four fields to the constructor.
///
/// Same shape as the medic-driver clone except the flag byte lives at
/// `+0x31`. Allocates through the pool at `POOL`; null stays null.
///
/// Original: 0x005dcf10 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dcf10(this: u32) -> u32 {
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
        let b31 = rd8(this + 0x31) as u32;
        lf_checker_rt::callee_thiscall!(CTOR, u32, new, w14, w18, this + 0x20, b31);
        new
    }
});
