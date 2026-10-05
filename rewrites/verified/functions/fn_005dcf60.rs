// original: 0x005dcf60 CTaskComplexDriveFireTruck::vf1

/// Clone a drive-fire-truck task, keeping references on two children.
///
/// Allocates through the pool at `POOL`; null stays null. Otherwise
/// base-constructs the new task, stamps `VTABLE`, copies the flag byte
/// at `+0x18` and the child pointers at `+0x14`/`+0x1c`, zeroes `+0x20`,
/// and reference-counts each non-null child.
///
/// Original: 0x005dcf60 (thiscall).
lf_checker_rt::export!(thiscall, rw_005dcf60(this: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0e84;
        const ALLOC: u32 = 1;
        const BASE_CTOR: u32 = 2;
        const ADDREF: u32 = 3;
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let new = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if new == 0 {
            return 0;
        }
        let b18 = rd8(this + 0x18);
        let w1c = rd32(this + 0x1c);
        let w14 = rd32(this + 0x14);
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, new);
        wr8(new + 0x18, b18);
        wr32(new, lf_checker_rt::relocated(VTABLE));
        wr32(new + 0x14, w14);
        wr32(new + 0x1c, w1c);
        wr32(new + 0x20, 0);
        if w14 != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, w14, new + 0x14);
        }
        let c1c = rd32(new + 0x1c);
        if c1c != 0 {
            lf_checker_rt::callee_thiscall!(ADDREF, u32, c1c, new + 0x1c);
        }
        new
    }
});
