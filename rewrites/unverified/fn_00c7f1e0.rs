// original: 0x00C7F1E0 CTaskComplexStationaryScenario::create_next_sub_task

/// Create the next stationary-scenario task. An active scenario with no task
/// flags and a scenario already at state `2` return null. Otherwise state
/// `+0x54` is cleared. A nonzero mode byte at `+0x24` allocates and constructs
/// a task, selecting `0.0` for type `0x35` and `-1.0` for other types; mode
/// zero allocates a task, initializes it, and writes its vtable and fields.
/// The method uses thiscall with one unused stack argument.
lf_checker_rt::export!(thiscall, rw_00c7f1e0(this: u32, _unused: u32) -> u32 {
    const ACTIVE_BYTE_OFFSET: u32 = 0x20;
    const TASK_FLAGS_OFFSET: u32 = 0x1C;
    const TASK_TYPE_OFFSET: u32 = 0x14;
    const MODE_BYTE_OFFSET: u32 = 0x24;
    const TASK_STATE_OFFSET: u32 = 0x54;
    const TASK_VARIANT_OFFSET: u32 = 0x65;
    const TASK_POOL_GLOBAL: u32 = 0x0167_E2A0;
    const TYPE_WITH_ZERO_BLEND: u32 = 0x35;
    const COMPLETED_STATE: u32 = 2;
    const ALLOCATE_TASK: u32 = 1;
    const CONSTRUCT_TASK: u32 = 2;
    const INITIALIZE_TASK: u32 = 3;

    let is_active = unsafe {
        (this.wrapping_add(ACTIVE_BYTE_OFFSET) as *const u8).read_unaligned()
    } != 0;
    let task_flags = unsafe {
        (this.wrapping_add(TASK_FLAGS_OFFSET) as *const u32).read_unaligned()
    };
    if is_active && task_flags == 0 {
        return 0;
    }

    let task_state = unsafe {
        (this.wrapping_add(TASK_STATE_OFFSET) as *const u32).read_unaligned()
    };
    if task_state == COMPLETED_STATE {
        return 0;
    }
    unsafe {
        (this.wrapping_add(TASK_STATE_OFFSET) as *mut u32).write_unaligned(0);
    }

    let task_pool = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned() };
    let mode = unsafe {
        (this.wrapping_add(MODE_BYTE_OFFSET) as *const u8).read_unaligned()
    };
    if mode != 0 {
        let task_type = unsafe {
            (this.wrapping_add(TASK_TYPE_OFFSET) as *const u32).read_unaligned()
        };
        let blend_bits = if task_type == TYPE_WITH_ZERO_BLEND {
            0.0f32.to_bits()
        } else {
            (-1.0f32).to_bits()
        };
        let allocated_task = lf_checker_rt::callee_thiscall!(ALLOCATE_TASK, u32, task_pool);
        if allocated_task == 0 {
            return 0;
        }
        let task_variant = unsafe {
            (this.wrapping_add(TASK_VARIANT_OFFSET) as *const u8).read_unaligned() as u32
        };
        return lf_checker_rt::callee_thiscall!(
            CONSTRUCT_TASK,
            u32,
            allocated_task,
            0,
            blend_bits,
            task_variant,
            0,
            0,
            0x0C,
        );
    }

    let task = lf_checker_rt::callee_thiscall!(ALLOCATE_TASK, u32, task_pool);
    if task == 0 {
        return 0;
    }
    let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_TASK, u32, task);
    let task_vtable = lf_checker_rt::relocated(0x00EB_391C);
    unsafe {
        (task as *mut u32).write_unaligned(task_vtable);
        (task.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        (task.wrapping_add(0x18) as *mut u8).write_unaligned(0);
        (task.wrapping_add(0x1C) as *mut u32).write_unaligned(0);
    }
    task
});
