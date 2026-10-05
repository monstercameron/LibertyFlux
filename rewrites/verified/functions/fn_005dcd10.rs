// original: 0x005dcd10 CTaskSimpleHandsUp::vf1

/// Clone a hands-up task: allocate from the task pool, construct, stamp vtable.
///
/// `this` is the source task (field `+0x2c` is forwarded). Allocates
/// a new task through the pool at `POOL`; a null allocation returns null.
/// Otherwise constructs the new task with the eight-word argument list
/// `(0, 2, 4.0, -4.0, field, 0x19d, template, 0)`, stamps `VTABLE` and
/// returns the new task.
///
/// Original: 0x005dcd10 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dcd10(this: u32) -> u32 {
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
        const VTABLE: u32 = 0xeb73cc;
        const KIND: u32 = 0x19d;
        const TMPL: u32 = 0xfe0d70;
        const F_LO: u32 = 0x40800000;
        const F_HI: u32 = 0xc0800000;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        let field = rd32(this + 0x2c);
        lf_checker_rt::callee_thiscall!(CTOR, u32, new, 0, 2, F_LO, F_HI, field, KIND,
            lf_checker_rt::relocated(TMPL), 0);
        wr32(new, lf_checker_rt::relocated(VTABLE));
        new
    }
});
