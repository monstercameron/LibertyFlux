// original: 0x00D013F0 CTaskSimplePlaySwatSignalAnim::vf1

/// Clone a swat-signal-anim task through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the signal word at `+0x28` to the clone routine (patched callee 2) and
/// returns its result.
///
/// Original: 0x00D013F0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d013f0(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        const SIGNAL: u32 = 0x28;

        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CLONE, u32, target, rd32(this + SIGNAL))
    }
});
