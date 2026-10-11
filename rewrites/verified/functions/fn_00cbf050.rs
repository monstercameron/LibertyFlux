// original: 0x00CBF050 ped_select_task_candidate
/// Ask the manager whether a task is already active. If not, read the task
/// word through the second input pointer, obtain a candidate value, and pass
/// the manager, task word, fixed threshold, and candidate to the filter. The
/// helper calls are intercepted by the contract. The cdecl method takes two
/// stack arguments.
lf_checker_rt::export!(cdecl, rw_00cbf050(
    list: *mut u8,
    task_ref: *const u32,
) -> u32 {
    const MANAGER_OFFSET: usize = 0x78;
    let manager = unsafe { list.add(MANAGER_OFFSET).cast::<u32>().read_unaligned() };
    let has_active_task = lf_checker_rt::callee_thiscall!(1, u32, manager, 0);
    if has_active_task != 0 {
        return has_active_task;
    }
    let task_word = unsafe { task_ref.read_unaligned() };
    let candidate = lf_checker_rt::callee_thiscall!(2, u32, list as u32);
    lf_checker_rt::callee_cdecl!(3, u32, manager, task_word, 0x0F, 0x40800000, candidate)
});
