// original: 0x00a80650 CSimpleNMShotTaskInfo::create_task

/// Create the shot task from its two stored values and resolved shooter.
///
/// The `thiscall` receiver stores constructor values at `+0x18` and `+0x1C`.
/// The method first allocates a task from the manager global. If allocation
/// fails it returns zero. Otherwise it resolves the shooter through a helper,
/// then forwards the reloaded `+0x18` word, the reloaded `+0x1C` word, and the
/// helper result to the constructor in that order. The constructor's `EAX` is
/// returned; all helpers are intercepted through the checker runtime.
lf_checker_rt::export!(thiscall, rw_00a80650(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const FIRST_TASK_VALUE: u32 = 0x18;
    const SECOND_TASK_VALUE: u32 = 0x1C;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const SHOOTER_ENTITY_LOOKUP: u32 = 2;
    const SHOT_TASK_CONSTRUCTOR: u32 = 3;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let shooter = lf_checker_rt::callee_thiscall!(SHOOTER_ENTITY_LOOKUP, u32, task_info);
    let first_value = unsafe {
        ((task_info.wrapping_add(FIRST_TASK_VALUE)) as *const u32).read_unaligned()
    };
    let second_value = unsafe {
        ((task_info.wrapping_add(SECOND_TASK_VALUE)) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        SHOT_TASK_CONSTRUCTOR,
        u32,
        task,
        first_value,
        second_value,
        shooter
    )
});
