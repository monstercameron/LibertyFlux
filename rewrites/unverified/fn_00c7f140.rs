// original: 0x00C7F140 CTaskComplexMoveBetweenPointsScenario::create_next_sub_task

/// Advance the move-between-points scenario. State `1` releases its pending
/// task and clears `+0x2C`; an inactive scenario with a zero task id returns
/// null. A ped-state helper and the runtime float threshold can suppress task
/// creation; otherwise the task-pool allocator and constructor are called in
/// order. The method uses thiscall with one ped pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00c7f140(this: u32, ped: u32) -> u32 {
    const TASK_ID_OFFSET: u32 = 0x14;
    const TASK_PARAMETER_OFFSET: u32 = 0x18;
    const TASK_FLAGS_OFFSET: u32 = 0x1C;
    const ACTIVE_BYTE_OFFSET: u32 = 0x20;
    const STATE_OFFSET: u32 = 0x28;
    const PENDING_TASK_OFFSET: u32 = 0x2C;
    const PED_READY_BYTE_OFFSET: u32 = 0xA60;
    const PED_STATE_OFFSET: u32 = 0x24;
    const TASK_STATUS_OFFSET: u32 = 0x50;
    const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
    const CURRENT_TIME_GLOBAL: u32 = 0x012D_DEAC;
    const TIME_LIMIT_GLOBAL: u32 = 0x00FE_87D0;
    const RELEASE_PENDING_TASK: u32 = 1;
    const GET_PED_TASK: u32 = 2;
    const ALLOCATE_TASK: u32 = 3;
    const CONSTRUCT_MOVE_TASK: u32 = 4;
    const READY_STATE: u32 = 1;
    const COMPLETED_STATE: u32 = 2;
    const REQUIRED_PED_STATE_BIT: u32 = 0x0800_0000;
    const REQUIRED_TASK_FLAG: u8 = 2;

    let state = unsafe { (this.wrapping_add(STATE_OFFSET) as *const u32).read_unaligned() };
    if state == READY_STATE {
        let pending_task = unsafe {
            (this.wrapping_add(PENDING_TASK_OFFSET) as *const u32).read_unaligned()
        };
        if pending_task != 0 {
            let _ = lf_checker_rt::callee_cdecl!(RELEASE_PENDING_TASK, u32, pending_task);
        }
        unsafe {
            (this.wrapping_add(PENDING_TASK_OFFSET) as *mut u32).write_unaligned(0);
        }
    }

    let is_active = unsafe {
        (this.wrapping_add(ACTIVE_BYTE_OFFSET) as *const u8).read_unaligned()
    } != 0;
    let task_id = unsafe { (this.wrapping_add(TASK_ID_OFFSET) as *const u32).read_unaligned() };
    if !is_active && task_id == 0 {
        return 0;
    }
    if unsafe { (this.wrapping_add(STATE_OFFSET) as *const u32).read_unaligned() }
        == COMPLETED_STATE
    {
        return 0;
    }
    unsafe {
        (this.wrapping_add(STATE_OFFSET) as *mut u32).write_unaligned(0);
    }

    let ped_task = lf_checker_rt::callee_cdecl!(GET_PED_TASK, u32, task_id);
    let ped_ready = unsafe {
        (ped.wrapping_add(PED_READY_BYTE_OFFSET) as *const u8).read_unaligned()
    } == 1;
    let current_time = unsafe {
        lf_checker_rt::global::<f32>(CURRENT_TIME_GLOBAL).read_unaligned()
    };
    let time_limit = unsafe {
        lf_checker_rt::global::<f32>(TIME_LIMIT_GLOBAL).read_unaligned()
    };
    let time_is_below_or_unordered = current_time.is_nan() || current_time < time_limit;
    let ped_task_has_flag = ped_task != 0
        && unsafe {
            (ped_task.wrapping_add(TASK_STATUS_OFFSET) as *const u8).read_unaligned()
                & REQUIRED_TASK_FLAG
                != 0
        };
    let ped_state = unsafe { (ped.wrapping_add(PED_STATE_OFFSET) as *const u32).read_unaligned() };
    if ped_ready
        && !time_is_below_or_unordered
        && ped_task_has_flag
        && ped_state & REQUIRED_PED_STATE_BIT != 0
    {
        return 0;
    }

    let task_pool = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned() };
    let allocated_task = lf_checker_rt::callee_thiscall!(ALLOCATE_TASK, u32, task_pool);
    if allocated_task == 0 {
        return 0;
    }

    let task_parameter = unsafe {
        (this.wrapping_add(TASK_PARAMETER_OFFSET) as *const u32).read_unaligned()
    };
    let task_flags = unsafe {
        (this.wrapping_add(TASK_FLAGS_OFFSET) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        CONSTRUCT_MOVE_TASK,
        u32,
        allocated_task,
        task_id,
        task_parameter,
        task_flags,
        1,
        0,
        1,
    )
});
