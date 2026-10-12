// original: 0x00D01420 CTaskSimplePlayUpperCombatAnim::vf1

/// Clone an upper-combat-anim task by tail-calling the clone routine.
///
/// Null from the task manager (patched callee 1) means no clone. Otherwise
/// the clone routine (tail-patched callee 2) runs on the manager's answer and
/// its result is returned directly.
///
/// Original: 0x00D01420 (thiscall, no stack words; ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00d01420(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CLONE, u32, target)
    }
});
