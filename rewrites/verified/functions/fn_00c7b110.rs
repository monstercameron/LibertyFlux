// original: 0x00c7b110 CTaskComplexDriveWanderForTime::vf1

/// Clone factory that installs the vtable and copies the timer itself.
/// Allocates through the heap singleton (failure returns 0), runs the
/// base constructor callee on the slot, then installs `VTABLE` and
/// copies the word at `this+0x14` to slot `+0x14` (a bitwise copy;
/// the original spills it through a stack scratch slot). Returns the
/// new pointer.
/// Original: 0x00c7b110 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7b110(this: u32) -> u32 {
    unsafe {
        const HEAP: u32 = 0x0167E2A0;
        const VTABLE: u32 = 0x00ED5E54;
        const OFF_TIMER: u32 = 0x14;
        const ALLOC: u32 = 1;
        const BASE_CTOR: u32 = 2;
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
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, slot);
        let timer = rd32(this.wrapping_add(OFF_TIMER));
        wr32(slot, lf_checker_rt::relocated(VTABLE));
        wr32(slot.wrapping_add(OFF_TIMER), timer);
        slot
    }
});
