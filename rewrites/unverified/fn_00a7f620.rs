// original: 0x00A7F620 CDeadInfo::vf10

/// Create the task represented by `CDeadInfo::vf10` through the task manager.
/// A null allocation returns zero. Otherwise the initializer receives
/// the task in ECX and the signed constant 0 as its one stack
/// argument; its EAX result is returned. This is a no-argument thiscall.
lf_checker_rt::export!(thiscall, rw_00A7F620(_this: u32) -> u32 {
    unsafe {
        const MANAGER_SLOT: u32 = 0x018B69B4;
        const ALLOC_TASK: u32 = 1;
        const INIT_TASK: u32 = 2;
        const INIT_VALUE: u32 = 0;

        let manager = lf_checker_rt::global::<u32>(MANAGER_SLOT).read();
        let task = lf_checker_rt::callee_thiscall!(ALLOC_TASK, u32, manager);
        if task == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(INIT_TASK, u32, task, INIT_VALUE)
    }
});
