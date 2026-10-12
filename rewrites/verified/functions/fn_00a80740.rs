// original: 0x00a80740 create_task_for_ped_by_info_id

/// Find the task info for a stage and invoke its virtual task-creation method.
///
/// This `stdcall` function receives a stage identifier and a ped argument.
/// The stage lookup returns either null or a task-info object. For null it
/// returns zero. Otherwise it loads the object's vtable, calls slot two with
/// the ped argument as a `thiscall`, and returns that method's `EAX` result.
/// The lookup and virtual method are intercepted with checker stand-ins.
lf_checker_rt::export!(stdcall, rw_00a80740(stage_id: u32, ped: u32) -> u32 {
    const TASK_INFO_LOOKUP: u32 = 1;
    const CREATE_TASK_VIRTUAL: u32 = 2;

    let task_info = lf_checker_rt::callee_stdcall!(TASK_INFO_LOOKUP, u32, stage_id);
    if task_info == 0 {
        return 0;
    }

    lf_checker_rt::callee_thiscall!(CREATE_TASK_VIRTUAL, u32, task_info, ped)
});
