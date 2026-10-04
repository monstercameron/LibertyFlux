// original: 0x00c9d7e0 CTaskSimpleIK::vf2

/// Refresh an IK task: raise the refresh flag and re-resolve the target.
///
/// Sets bit 0 of the flag word at `+0x5c`, runs the parameter refresh
/// helper (callee 1), then re-resolves the cached target word at `+0x24`
/// through the target helper (callee 2). The stack word is ignored. No
/// value is returned.
///
/// Original: 0x00c9d7e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c9d7e0(this: u32, _ignored: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const FLAGS: u32 = 0x5c;
        const TARGET: u32 = 0x24;
        const REFRESH_BIT: u32 = 0x0000_0001;
        wr32(this + FLAGS, rd32(this + FLAGS) | REFRESH_BIT);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this, rd32(this + TARGET));
        0
    }
});
