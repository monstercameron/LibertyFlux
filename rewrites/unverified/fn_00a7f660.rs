// original: 0x00A7F660 CSimpleNMExplosionTaskInfo::vf10

/// Create the task represented by `CSimpleNMExplosionTaskInfo::vf10` through the task manager.
/// Read the manager pointer from its shared global slot, ask it for a
/// task object, return zero when allocation fails, and otherwise tail
/// forward the new object to this task's initializer. The initializer's
/// EAX result is returned unchanged. This is a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00A7F660(_this: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT: u32 = 0x018B69B4;
        const ALLOC_TASK: u32 = 1;
        const INIT_TASK: u32 = 2;

        let manager = lf_checker_rt::global::<u32>(MANAGER_SLOT).read();
        let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
        if task == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(INIT_TASK, u32, task)
    }
});
