// original: 0x00A7F720 CSimpleSidewaysDiveTaskInfo::create_default_task
/// Create a default task for CSimpleSidewaysDiveTaskInfo. Read the manager pointer, request a task, return null on allocation failure, and otherwise tail-call the sideways-dive initializer and forward its result.
lf_checker_rt::export!(thiscall, rw_00a7f720(_this: u32) -> u32 {
    const MANAGER_SLOT: u32 = 0x018B69B4;
    const ALLOC_TASK: u32 = 22;
    const INIT_DIVE_TASK: u32 = 25;
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_SLOT).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
    if task == 0 {
        return 0;
    }
    lf_checker_rt::callee_thiscall!(INIT_DIVE_TASK, u32, task)
});
