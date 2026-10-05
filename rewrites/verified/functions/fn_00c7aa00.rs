// original: 0x00c7aa00 CTaskComplexWanderingScenario::vf0

/// Deleting destructor: destroy the task, then free it when asked.
/// Runs the class destructor callee on `this`, then when bit 0 of
/// `flags` is set releases the object through the heap singleton
/// (`FREE` callee with the heap pointer in ECX and `this` pushed).
/// Returns `this` either way.
/// Original: 0x00c7aa00 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7aa00(this: u32, flags: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const CLASS_DTOR: u32 = 1;
        const FREE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_thiscall!(CLASS_DTOR, u32, this);
        if (flags & 1) != 0 {
            let heap = rd32(lf_checker_rt::relocated(HEAP));
            lf_checker_rt::callee_thiscall!(FREE, u32, heap, this);
        }
        this
    }
});
