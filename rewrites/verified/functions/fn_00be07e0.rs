// original: 0x00be07e0 CTaskComplexSitIdle::vf1
/// Ask the manager to create an idle task with the words at `this + 0x2c`, `+0x14` and `+0x18`, followed by the all-ones variation value and two zero words. A null manager returns zero; otherwise the helper's EAX value is returned. Calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00be07e0(this: u32) -> u32 {
    const TASK_MANAGER_POINTER_VA: u32 = 0x0167_E2A0;
    const TASK_MANAGER_LOOKUP: u32 = 1;
    const START_IDLE: u32 = 2;
    const NO_VARIATION: u32 = u32::MAX;
    let read_word = |address: u32| unsafe { (address as *const u32).read_unaligned() };
    let task_manager_context = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER_POINTER_VA).read_unaligned() };
    let manager_context = lf_checker_rt::callee_thiscall!(TASK_MANAGER_LOOKUP, u32, task_manager_context);
    if manager_context == 0 { 0 } else {
        lf_checker_rt::callee_thiscall!(START_IDLE, u32, manager_context,
            read_word(this.wrapping_add(0x2C)), read_word(this.wrapping_add(0x14)),
            read_word(this.wrapping_add(0x18)), NO_VARIATION, 0, 0)
    }
});
