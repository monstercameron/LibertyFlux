// original: 0x00D013B0 CTaskSimplePlayAnimAndSlideOutOfCover::vf1

/// Clone a slide-out-of-cover task through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the cover words at `+0x14`/`+0x18`, the blend word at `+0x1c` (bit pattern),
/// the source block at `+0x20` and the stance byte at `+0x54` to the clone
/// routine (patched callee 2) and returns its result.
///
/// Original: 0x00D013B0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d013b0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;

        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CLONE, u32, target,
            rd32(this + 0x14), rd32(this + 0x18), rd32(this + 0x1c),
            this + 0x20, ((this + 0x54) as *const u8).read() as u32)
    }
});
