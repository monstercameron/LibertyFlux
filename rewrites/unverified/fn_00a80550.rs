// original: 0x00a80550 CSimpleNMBraceTaskInfo::create_task

/// Create the brace task after the task pool supplies storage.
///
/// The `thiscall` receiver is the task-info object. The method reads the task
/// pool manager from the image global and allocates one task. If allocation
/// fails it returns zero. Otherwise it queries the brace entity through the
/// receiver, passes that result to the task constructor, and returns the
/// constructor's `EAX` value. Each helper call is represented by a checker
/// runtime call so its order and arguments are compared.
lf_checker_rt::export!(thiscall, rw_00a80550(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const BRACE_ENTITY_LOOKUP: u32 = 2;
    const BRACE_TASK_CONSTRUCTOR: u32 = 3;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let brace_entity = lf_checker_rt::callee_thiscall!(BRACE_ENTITY_LOOKUP, u32, task_info);
    lf_checker_rt::callee_thiscall!(BRACE_TASK_CONSTRUCTOR, u32, task, brace_entity)
});
