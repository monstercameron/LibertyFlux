// original: 0x00A7F6D0 CSimpleNMJumpRollTaskInfo::create_task
/// Create a CSimpleNMJumpRollTaskInfo task. Read the manager pointer from the runtime manager slot, request a task, and return null if allocation fails. Otherwise run the task initializer, install the relocated task vtable, and return the allocated task.
lf_checker_rt::export!(thiscall, rw_00a7f6d0(_this: u32) -> u32 {
    const MANAGER_SLOT: u32 = 0x018B69B4;
    const ALLOC_TASK: u32 = 22;
    const INIT_TASK: u32 = 23;
    const VTABLE: u32 = 0x00EA1FCC;
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_SLOT).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
    if task == 0 {
        return 0;
    }
    let _ = lf_checker_rt::callee_thiscall!(INIT_TASK, u32, task);
    unsafe { (task as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)); }
    task
});
