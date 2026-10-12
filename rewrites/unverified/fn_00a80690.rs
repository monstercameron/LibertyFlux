// original: 0x00a80690 CSimpleSidewaysDiveTaskInfo::create_task

/// Allocate a sideways-dive task and initialize it from the stored task word.
///
/// The `thiscall` receiver holds the source task-info object. This method reads
/// the task-pool manager pointer and allocates a task. It returns zero on
/// allocation failure; otherwise it loads the word at receiver offset `+0x18`,
/// passes that value to the task constructor, and returns the constructor's
/// `EAX` result.
lf_checker_rt::export!(thiscall, rw_00a80690(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const TASK_VALUE: u32 = 0x18;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const SIDEWAYS_DIVE_CONSTRUCTOR: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let task_value = unsafe {
        ((task_info.wrapping_add(TASK_VALUE)) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(SIDEWAYS_DIVE_CONSTRUCTOR, u32, task, task_value)
});
