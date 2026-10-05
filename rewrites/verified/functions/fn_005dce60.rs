// original: 0x005dce60 CTaskComplexTreatAccident::vf1

/// Clone a treat-accident task, keeping a reference on the child at +0x14.
///
/// Allocates through the pool at `POOL`; null stays null. Otherwise
/// base-constructs the new task, stamps `VTABLE`, copies the child
/// pointer at `+0x14` and reference-counts it when non-null.
///
/// Original: 0x005dce60 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dce60(this: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0dd4;
        const ALLOC: u32 = 1;
        const BASE_CTOR: u32 = 2;
        const ADDREF: u32 = 3;
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        let child = rd32(this + 0x14);
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, new);
        wr32(new, lf_checker_rt::relocated(VTABLE));
        wr32(new + 0x14, child);
        if child != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, child, new + 0x14);
        }
        new
    }
});
