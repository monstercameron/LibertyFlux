// original: 0x00C7F0F0 CTaskComplexMobileChatScenario::create_next_sub_task

/// Create the scenario's next task only when the receiver's state word at
/// `+0x14` equals `0x5D`. The helper receives the second argument advanced
/// by `0x2B0`; its return value is ignored, and this method always returns
/// null. The method uses thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00c7f0f0(this: u32, scenario: u32) -> u32 {
    const STATE_OFFSET: u32 = 0x14;
    const READY_STATE: u32 = 0x5D;
    const TASK_MANAGER_OFFSET: u32 = 0x2B0;
    const CREATE_TASK: u32 = 1;

    let state = unsafe { (this.wrapping_add(STATE_OFFSET) as *const u32).read_unaligned() };
    if state == READY_STATE {
        let task_manager = scenario.wrapping_add(TASK_MANAGER_OFFSET);
        let _ = lf_checker_rt::callee_thiscall!(CREATE_TASK, u32, task_manager);
    }
    0
});
