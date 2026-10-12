// original: 0x00a7f740 CSimpleThrowProjectileInfo::create_default_task

/// Allocate and initialize the default projectile task.
///
/// This `thiscall` method ignores its receiver. It loads the task-pool manager
/// pointer from the image global and asks that manager for an object. A null
/// allocation returns zero. Otherwise it calls the task constructor with the
/// two constant arguments `0` and `3`, returning the constructor's full `EAX`
/// result. The pointers here are 32-bit ABI values because the original image
/// and checker worker use the 32-bit game layout.
lf_checker_rt::export!(thiscall, rw_00a7f740(_receiver: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const PROJECTILE_TASK_CONSTRUCTOR: u32 = 2;
    const DEFAULT_TASK_KIND: u32 = 3;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    lf_checker_rt::callee_thiscall!(PROJECTILE_TASK_CONSTRUCTOR, u32, task, 0, DEFAULT_TASK_KIND)
});
