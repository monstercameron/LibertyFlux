// original: 0x00a80760 CDeadInfo::create_task_for_ped

/// Apply the dead-task iterator callback, allocate a task, then construct it.
///
/// This `thiscall` receiver is a dead-task info object and the stack argument
/// is the ped pointer. First the method calls the iterator helper with the ped
/// in ECX and the exact float bits for `4.0` on the stack. It then allocates
/// from the task-pool manager in the image global. A null allocation returns
/// zero. Otherwise it passes the second image global, the zero-extended byte
/// at receiver offset `+0x18`, and zero to the constructor in that order, and
/// returns the constructor's `EAX`. Every callee is a checker runtime call.
lf_checker_rt::export!(thiscall, rw_00a80760(task_info: u32, ped: u32) -> u32 {
    const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
    const TASK_CONSTRUCTOR_MODE_VA: u32 = 0x011735B4;
    const TASK_KIND: u32 = 0x18;
    const PED_ITERATOR_STEP_BITS: u32 = 0x40800000;
    const PED_ITER_APPLY_FLOAT: u32 = 1;
    const TASK_POOL_ALLOCATOR: u32 = 2;
    const DEAD_TASK_CONSTRUCTOR: u32 = 3;

    let _iterator_result = lf_checker_rt::callee_thiscall!(
        PED_ITER_APPLY_FLOAT,
        u8,
        ped,
        PED_ITERATOR_STEP_BITS
    );

    let manager = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned()
    };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let task_kind = u32::from(unsafe {
        ((task_info.wrapping_add(TASK_KIND)) as *const u8).read()
    });
    let constructor_mode = unsafe {
        lf_checker_rt::global::<u32>(TASK_CONSTRUCTOR_MODE_VA).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        DEAD_TASK_CONSTRUCTOR,
        u32,
        task,
        constructor_mode,
        task_kind,
        0u32
    )
});
