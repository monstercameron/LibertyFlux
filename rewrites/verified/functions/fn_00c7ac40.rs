// original: 0x00c7ac40 CTaskComplexWaitForSeatToBeFree::vf21

/// True when the seat handle is gone or its check reports free.
/// A null handle at `this+0x18` ends the wait (returns 1). Else the
/// seat-check callee runs on the handle with the word at `this+0x14`,
/// and the result is 1 iff the callee returned 0. The stack word is
/// popped but never read.
/// Original: 0x00c7ac40 (thiscall, one ignored stack word).
lf_checker_rt::export!(thiscall, rw_00c7ac40(this: u32, _a0: u32) -> u32 {
    unsafe {
        const OFF_ARG: u32 = 0x14;
        const OFF_HANDLE: u32 = 0x18;
        const CHECK: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let handle = rd32(this.wrapping_add(OFF_HANDLE));
        if handle == 0 {
            return 1;
        }
        let arg = rd32(this.wrapping_add(OFF_ARG));
        let r = lf_checker_rt::callee_thiscall!(CHECK, u32, handle, arg);
        if r == 0 { 1 } else { 0 }
    }
});
