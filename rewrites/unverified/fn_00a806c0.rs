// original: 0x00a806c0 CSimpleThrowProjectileInfo::create_task

/// Allocate and initialize a projectile task from its task-info payload.
///
/// The `thiscall` receiver is the task-info object. The method loads the task
/// pool manager from the image global and allocates storage. On allocation
/// failure it returns zero. Otherwise it passes zero followed by the receiver's
/// `+0x18` payload word to the constructor and returns that call's `EAX` value.
/// The manager and both callees are represented by checker runtime calls.
lf_checker_rt::export!(thiscall, rw_00a806c0(task_info: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const PAYLOAD_WORD: u32 = 0x18;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const PROJECTILE_TASK_CONSTRUCTOR: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let payload = unsafe {
        ((task_info.wrapping_add(PAYLOAD_WORD)) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        PROJECTILE_TASK_CONSTRUCTOR,
        u32,
        task,
        0u32,
        payload
    )
});
