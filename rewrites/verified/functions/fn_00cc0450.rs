// original: 0x00CC0450 CPedMoveBlendOnFoot::vf22
/// Return one in AL when the manager yields a task of type 0x26, 0x27,
/// 0x30, 0x31, 0x35 or 0x36; continue through tasks of other types and return
/// zero when the manager is exhausted. Task type is at task offset 0x0C.
lf_checker_rt::export!(thiscall, rw_00cc0450(_this: u32, manager: u32) -> u32 {
    const ACCEPTED_TASK_TYPES: [u32; 6] = [0x26,0x27,0x30,0x31,0x35,0x36];
    const FIRST_TASK_INDEX: u32 = 0;
    const TASK_QUERY_LIMIT: u32 = 2;
    let mut task = lf_checker_rt::callee_thiscall!(1, u32, manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    loop {
        if task == 0 {
            return 0;
        }
        let task_type = unsafe { (task as *const u8).add(0x0C).cast::<u32>().read_unaligned() };
        if ACCEPTED_TASK_TYPES.contains(&task_type) {
            return 1;
        }
        task = lf_checker_rt::callee_thiscall!(2, u32, manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    }
});
