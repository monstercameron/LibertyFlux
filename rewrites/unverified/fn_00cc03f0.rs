// original: 0x00CC03F0 CPedMoveBlendOnFoot::vf21
/// Search the task manager until task type 0x12 through 0x1B is found, then
/// return that task pointer; return zero when the manager yields no task.
/// Task type is the dword at task offset 0x0C. The manager is a stack argument.
lf_checker_rt::export!(thiscall, rw_00cc03f0(_this: u32, manager: u32) -> u32 {
    const ACCEPTED_TASK_TYPES: [u32; 10] = [0x12,0x13,0x14,0x15,0x16,0x17,0x18,0x19,0x1A,0x1B];
    let mut task = lf_checker_rt::callee_thiscall!(1, u32, manager, 0, 2);
    loop {
        if task == 0 {
            return 0;
        }
        let task_type = unsafe { (task as *const u8).add(0x0C).cast::<u32>().read_unaligned() };
        if ACCEPTED_TASK_TYPES.contains(&task_type) {
            return task;
        }
        task = lf_checker_rt::callee_thiscall!(2, u32, manager, 0, 2);
    }
});
