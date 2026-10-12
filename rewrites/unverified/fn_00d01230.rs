// original: 0x00D01230 CTaskComplexSetAndGuardArea::vf1

/// Clone a set-and-guard-area task through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the area block at `+0x20`, three coordinate words at `+0x60`/`+0x64`/`+0x68`
/// (bit patterns, moved not computed), the radius word at `+0x6c`, the guard
/// block at `+0x30` and the flag bytes at `+0x70`/`+0x71` to the clone routine
/// (patched callee 2) and returns its result.
///
/// Original: 0x00D01230 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d01230(this: u32) -> u32 {
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
            this + 0x20, rd32(this + 0x60), rd32(this + 0x64), rd32(this + 0x68),
            rd32(this + 0x6c), this + 0x30,
            ((this + 0x70) as *const u8).read() as u32,
            ((this + 0x71) as *const u8).read() as u32)
    }
});
