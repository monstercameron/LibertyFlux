// original: 0x00a80580 CSimpleNMExplosionTaskInfo::create_task

/// Allocate an explosion task and initialize it from the embedded payload.
///
/// The `thiscall` receiver holds the task-info object. This method reads the
/// task-pool manager pointer, allocates a task, and returns zero on allocation
/// failure. On success it passes the address of the payload at receiver offset
/// `+0x20` to the constructor and returns the constructor's `EAX` value.
lf_checker_rt::export!(thiscall, rw_00a80580(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const EMBEDDED_PAYLOAD: u32 = 0x20;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const EXPLOSION_TASK_CONSTRUCTOR: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let payload = task_info.wrapping_add(EMBEDDED_PAYLOAD);
    lf_checker_rt::callee_thiscall!(EXPLOSION_TASK_CONSTRUCTOR, u32, task, payload)
});
