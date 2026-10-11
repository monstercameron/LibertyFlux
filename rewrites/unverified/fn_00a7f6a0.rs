// original: 0x00A7F6A0 CSimpleNMHighFallTaskInfo::vf9

/// Construct `CSimpleNMHighFallTaskInfo::vf9` through the task manager. A null allocation
/// returns zero. Otherwise run the initializer, set the allocated
/// record's vtable pointer, clear its dword at +0x1c, and return the
/// record address. This is a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00A7F6A0(_this: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT: u32 = 0x018B69B4;
        const ALLOC_TASK: u32 = 1;
        const INIT_TASK: u32 = 2;
        const TASK_VTABLE: u32 = 0x00EA1EEC;
        const RESET_FIELD: u32 = 0x1c;

        let manager = lf_checker_rt::global::<u32>(MANAGER_SLOT).read();
        let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
        if task == 0 {
            return 0;
        }
        let _ = lf_checker_rt::callee_thiscall!(INIT_TASK, u32, task);
        (task as *mut u32).write(lf_checker_rt::relocated(TASK_VTABLE));
        (task.wrapping_add(RESET_FIELD) as *mut u32).write(0);
        task
    }
});
