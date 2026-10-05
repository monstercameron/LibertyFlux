// original: 0x00c7b1f0 CTaskComplexMobileMakeCall::vf1

/// Clone factory forwarding an embedded member pointer.
/// Allocates through the heap singleton (failure returns 0). Else the
/// constructor callee runs with (`this+0x20`, `[this+0x14]`,
/// `[this+0x40]`): the first argument points into the source object
/// itself. Returns the new pointer.
/// Original: 0x00c7b1f0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7b1f0(this: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const OFF_EMBEDDED: u32 = 0x20;
        const OFF_A: u32 = 0x14;
        const OFF_B: u32 = 0x40;
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
        let a = rd32(this.wrapping_add(OFF_A));
        let b = rd32(this.wrapping_add(OFF_B));
        lf_checker_rt::callee_thiscall!(CTOR, u32, slot, this.wrapping_add(OFF_EMBEDDED), a, b)
    }
});
