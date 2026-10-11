// original: 0x00be08e0 CTaskComplexWaitAtAttractor::vf1
/// Create a wait-at-attractor task with the two words at `this + 0x14` and `this + 0x18` when the global manager lookup succeeds. A null manager returns zero; otherwise the helper's EAX value is propagated. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be08e0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_WAIT_TASK: u32 = 2;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(CREATE_WAIT_TASK, u32, manager_context,
            read_word(this.wrapping_add(0x14)), read_word(this.wrapping_add(0x18)))
    }
});
