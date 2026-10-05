// original: 0x00c7ade0 CTaskComplexWaitTillItsOkToStop::vf21

/// True when the stop-ok check on the owner's extension passes.
/// A null owner link at `[owner+0xb30]` returns 0. Else the check
/// callee runs on link+`0xdd4` and the result is 1 iff its low byte
/// is non-zero. Only the low byte of EAX is set. ECX (`this`) is
/// clobbered, never read.
/// Original: 0x00c7ade0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7ade0(this: u32, owner: u32) -> u32 {
    unsafe {
        const OFF_OWNER: u32 = 0xb30;
        const EXT_BIAS: u32 = 0xdd4;
        const CHECK: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let link = rd32(owner.wrapping_add(OFF_OWNER));
        if link == 0 {
            return 0;
        }
        let r = lf_checker_rt::callee_thiscall!(CHECK, u32, link.wrapping_add(EXT_BIAS));
        if (r & 0xFF) != 0 { 1 } else { 0 }
    }
});
