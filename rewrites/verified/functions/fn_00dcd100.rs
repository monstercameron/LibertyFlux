// original: 0x00DCD100 CTaskSimpleReloadGun::vf0
/// Runs the task's destructor, conditionally releases the global reload
/// manager with the task as its argument, and returns the original task
/// pointer. The manager is read from the relocated image global; both calls
/// are scripted by the checker.
lf_checker_rt::export!(thiscall, rw_00dcd100(task: u32, release_flag: u32) -> u32 {
    const TASK_DESTROY_ID: u32 = 1;
    const MANAGER_RELEASE_ID: u32 = 2;
    const MANAGER_GLOBAL_VA: u32 = 0x0167e2a0;

    let _ = lf_checker_rt::callee_thiscall!(TASK_DESTROY_ID, u32, task);
    if release_flag & 1 != 0 {
        // SAFETY: the checker maps the original image's global data for this read.
        let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read() };
        let _ = lf_checker_rt::callee_thiscall!(MANAGER_RELEASE_ID, u32, manager, task);
    }
    task
});
