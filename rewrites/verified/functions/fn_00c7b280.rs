// original: 0x00c7b280 CTaskComplexSeatedScenario::vf1

/// Clone factory copying two flag bytes; its null path faults.
/// Allocates through the heap singleton. On success the constructor
/// callee builds the object from fields `+0x14/0x18/0x1c` and the flag
/// bytes at `+0x25` and `+0x28` are copied from `this` to the clone,
/// which is returned. On allocation failure the original keeps going
/// with a null pointer and faults writing address `0x25`; the rewrite
/// does the same so the fault matches.
/// Original: 0x00c7b280 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7b280(this: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const OFF_A: u32 = 0x14;
        const OFF_B: u32 = 0x18;
        const OFF_C: u32 = 0x1c;
        const OFF_FLAG0: u32 = 0x25;
        const OFF_FLAG1: u32 = 0x28;
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
            let flag0 = (this.wrapping_add(OFF_FLAG0) as *const u8).read();
            (OFF_FLAG0 as *mut u8).write(flag0);
            let flag1 = (this.wrapping_add(OFF_FLAG1) as *const u8).read();
            (OFF_FLAG1 as *mut u8).write(flag1);
            return 0;
        }
        let a = rd32(this.wrapping_add(OFF_A));
        let b = rd32(this.wrapping_add(OFF_B));
        let c = rd32(this.wrapping_add(OFF_C));
        let r = lf_checker_rt::callee_thiscall!(CTOR, u32, slot, a, b, c);
        (r.wrapping_add(OFF_FLAG0) as *mut u8).write((this.wrapping_add(OFF_FLAG0) as *const u8).read());
        (r.wrapping_add(OFF_FLAG1) as *mut u8).write((this.wrapping_add(OFF_FLAG1) as *const u8).read());
        r
    }
});
