// original: 0x00c7b160 CTaskComplexDrivingScenario::vf1

/// Clone factory: allocate a fresh task and construct it from `this`.
/// Allocates through the heap singleton; allocation failure returns
/// 0. Else the constructor callee builds the new object from fields
/// `+0x14`, `+0x24` of `this`, and the new pointer is returned.
/// Original: 0x00c7b160 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7b160(this: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let heap = rd32(lf_checker_rt::relocated(HEAP));
        let slot = lf_checker_rt::callee_thiscall!(ALLOC, u32, heap);
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, slot, rd32(this.wrapping_add(0x14)), rd32(this.wrapping_add(0x24)))
    }
});
