// original: 0x00CBFF50 update_tasks_beyond_threshold
/// Walk the task list from the manager, update matching task identifiers when
/// their scripted f32 distance is greater than the input threshold, and stop
/// when the next-task helper returns null. NaN comparisons do not update.
lf_checker_rt::export!(thiscall, rw_00cbff50(_this: u32, manager: u32, target_id: u32, threshold: f32) -> u32 {
    const TASK_ID_OFFSET: usize = 0x10;
    const FIRST_TASK_INDEX: u32 = 0;
    const TASK_QUERY_LIMIT: u32 = 2;
    let mut task = lf_checker_rt::callee_thiscall!(1, u32, manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    while task != 0 {
        let task_id = unsafe { (task as *const u8).add(TASK_ID_OFFSET).cast::<u32>().read_unaligned() };
        if task_id == target_id {
            let distance = lf_checker_rt::callee_thiscall!(2, f32, task);
            if distance > threshold {
                let _ = lf_checker_rt::callee_thiscall!(3, u32, task, threshold.to_bits());
            }
        }
        task = lf_checker_rt::callee_thiscall!(4, u32, manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    }
    0
});
