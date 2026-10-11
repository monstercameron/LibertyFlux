// original: 0x00be0840 CTaskComplexUseEffect::vf1
/// Create an effect task with the words at `this + 0x14` and `this + 0x18` when the manager lookup succeeds. A null manager returns zero; otherwise the helper's EAX result is propagated. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be0840(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const CREATE_EFFECT_TASK: u32 = 2;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(CREATE_EFFECT_TASK, u32, manager_context,
            read_word(this.wrapping_add(0x14)), read_word(this.wrapping_add(0x18)))
    }
});
