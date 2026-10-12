// original: 0x00A7F700 CSimpleNMShotTaskInfo::create_default_task
/// Create a default task for CSimpleNMShotTaskInfo. Read the manager pointer, request a task, return null on allocation failure, and otherwise tail-call the shot-task initializer and forward its result.
lf_checker_rt::export!(thiscall, rw_00a7f700(_this: u32) -> u32 {
    const MANAGER_SLOT: u32 = 0x018B69B4;
    const ALLOC_TASK: u32 = 22;
    const INIT_SHOT_TASK: u32 = 24;
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_SLOT).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
    if task == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(INIT_SHOT_TASK, u32, task)
});
