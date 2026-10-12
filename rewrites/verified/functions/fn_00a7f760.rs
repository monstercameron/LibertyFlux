// original: 0x00a7f760 CSitDownIdleThenStandTaskInfo::create_default_task

/// Allocate and initialize the default sit-down task.
///
/// This `thiscall` method ignores its receiver. It reads the task-pool manager
/// pointer from the image global, asks that manager for an object, and returns
/// zero if the allocation fails. Otherwise it forwards the allocated object
/// to the task initializer and returns the initializer's `EAX` result. The
/// original reaches that initializer with a tail jump; this Rust call uses the
/// checker stand-in for the same target.
lf_checker_rt::export!(thiscall, rw_00a7f760(_receiver: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const TASK_POOL_ALLOCATOR: u32 = 1;
    const DEFAULT_TASK_INITIALIZER: u32 = 2;

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    lf_checker_rt::callee_thiscall!(DEFAULT_TASK_INITIALIZER, u32, task)
});
