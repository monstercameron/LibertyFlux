// original: 0x00a805f0 CSimpleNMOnFireTaskInfo::create_task

/// Allocate an on-fire task, run its base initializer, and install its vtable.
///
/// This zero-stack-argument `thiscall` method gets the task-pool manager from
/// the image global and allocates an object. Allocation failure returns zero.
/// On success it invokes the base initializer, writes the relocated on-fire
/// task vtable pointer into the object's first word, and returns the object
/// pointer. The constructor's return value is ignored.
lf_checker_rt::export!(thiscall, rw_00a805f0(_receiver: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const ON_FIRE_TASK_VTABLE_VA: u32 = 0x00EA1F24;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const TASK_BASE_INITIALIZER: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let _initialized = lf_checker_rt::callee_thiscall!(TASK_BASE_INITIALIZER, u32, task);
    unsafe {
        (task as *mut u32).write_unaligned(lf_checker_rt::relocated(ON_FIRE_TASK_VTABLE_VA));
    }
    task
});
