// original: 0x00a804d0 CSimpleMeleeActionResultTaskInfo::create_task

/// Create the melee action-result task when the task-info fields allow it.
///
/// The receiver supplies an optional network-object pointer at `+0x18`, a
/// task-data word at `+0x1C`, and a byte flag at `+0x20`. If the network
/// object's word at `+0x6C` is nonzero, its network ID is queried and reduced
/// to the low unsigned 16 bits. A zero task-data word or a failed task-pool
/// allocation returns zero. Otherwise the constructor receives the ID, the
/// reloaded task-data word, zero, and the flag, in that order; its `EAX` is
/// returned. All helpers are intercepted through the checker runtime.
lf_checker_rt::export!(thiscall, rw_00a804d0(task_info: u32) -> u32 {
    const NETWORK_OBJECT_FIELD: u32 = 0x18;
    const NETWORK_OBJECT_ID_FIELD: u32 = 0x6C;
    const TASK_DATA_FIELD: u32 = 0x1C;
    const TASK_FLAG_FIELD: u32 = 0x20;
    const TASK_POOL_MANAGER_VA: u32 = 0x018B69B4;
    const NETWORK_ID_QUERY: u32 = 1;
    const TASK_POOL_ALLOCATOR: u32 = 2;
    const MELEE_TASK_CONSTRUCTOR: u32 = 3;

    let network_object = unsafe {
        ((task_info.wrapping_add(NETWORK_OBJECT_FIELD)) as *const u32).read_unaligned()
    };
    let network_id = if network_object == 0 {
        0
    } else {
        let network_object_id = unsafe {
            ((network_object.wrapping_add(NETWORK_OBJECT_ID_FIELD)) as *const u32)
                .read_unaligned()
        };
        if network_object_id == 0 {
            0
        } else {
            (lf_checker_rt::callee_thiscall!(NETWORK_ID_QUERY, u32, network_object_id) as u16)
                as u32
        }
    };

    let task_data_for_test = unsafe {
        ((task_info.wrapping_add(TASK_DATA_FIELD)) as *const u32).read_unaligned()
    };
    if task_data_for_test == 0 {
        return 0;
    }

    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned() };
    let task = lf_checker_rt::callee_thiscall!(TASK_POOL_ALLOCATOR, u32, manager);
    if task == 0 {
        return 0;
    }

    let flag = unsafe { ((task_info.wrapping_add(TASK_FLAG_FIELD)) as *const u8).read() } as u32;
    let task_data_for_constructor = unsafe {
        ((task_info.wrapping_add(TASK_DATA_FIELD)) as *const u32).read_unaligned()
    };
    lf_checker_rt::callee_thiscall!(
        MELEE_TASK_CONSTRUCTOR,
        u32,
        task,
        network_id,
        task_data_for_constructor,
        0,
        flag
    )
});
