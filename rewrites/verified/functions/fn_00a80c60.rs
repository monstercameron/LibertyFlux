// original: 0x00a80c60 CMedicTaskInfo::create_task_for_ped

/// Create the medic combat task when the target and source task qualify.
///
/// The `thiscall` receiver must have the expected task-info kind and a source
/// task pointer at `+0x14`. The ped argument supplies the target state: its byte
/// at `+0x210` gates status checks at `+0xA74`, and its pointer at `+0x224` must
/// be non-null. The source task's word at `+0x28` must have masked value
/// `0xC0` under mask `0x3C0`. A target helper is queried with IDs `0x772` and
/// `0x770`; if the first query returns zero, an apply helper is called. The
/// task-pool manager comes from the image global. A successful allocation is
/// initialized with the source task pointer and zero, and a positive second
/// query sets bit `0x08` at result-task offset `+0x60`. Each image helper is
/// called through a checker runtime stand-in.
lf_checker_rt::export!(thiscall, rw_00a80c60(task_info: u32, ped: u32) -> u32 {
    const EXPECTED_INFO_KIND: u32 = 0x76C;
    const INFO_KIND_OFFSET: u32 = 0x04;
    const SOURCE_TASK_OFFSET: u32 = 0x14;
    const PED_HAS_STATUS_OFFSET: u32 = 0x210;
    const PED_STATUS_OFFSET: u32 = 0xA74;
    const PED_TARGET_OFFSET: u32 = 0x224;
    const SOURCE_TASK_FLAGS_OFFSET: u32 = 0x28;
    const TARGET_STATE_OFFSET: u32 = 0x2E0;
    const STATUS_MASK: u32 = 0x3C0;
    const REQUIRED_STATUS: u32 = 0xC0;
    const RESULT_FLAGS_OFFSET: u32 = 0x60;
    const MEDIC_FLAG: u32 = 0x08;
    const TASK_POOL_MANAGER_VA: u32 = 0x0167E2A0;
    const TARGET_STATE_QUERY: u32 = 1;
    const APPLY_MEDIC_TASK: u32 = 2;
    const TASK_POOL_ALLOCATOR: u32 = 3;
    const MEDIC_TASK_CONSTRUCTOR: u32 = 4;

    let info_kind = unsafe {
        ((task_info.wrapping_add(INFO_KIND_OFFSET)) as *const u32).read_unaligned()
    };
    if info_kind != EXPECTED_INFO_KIND {
        return 0;
    }

    let has_status = unsafe {
        ((ped.wrapping_add(PED_HAS_STATUS_OFFSET)) as *const u8).read()
    };
    if has_status != 0 {
        let status = unsafe {
            ((ped.wrapping_add(PED_STATUS_OFFSET)) as *const u32).read_unaligned()
        };
        if status == 1 || status == 2 {
            return 0;
        }
    }

    let target_state = unsafe {
        ((ped.wrapping_add(PED_TARGET_OFFSET)) as *const u32).read_unaligned()
    };
    if target_state == 0 {
        return 0;
    }

    let source_task = unsafe {
        ((task_info.wrapping_add(SOURCE_TASK_OFFSET)) as *const u32).read_unaligned()
    };
    if source_task == 0 {
        return 0;
    }

    let source_flags = unsafe {
        ((source_task.wrapping_add(SOURCE_TASK_FLAGS_OFFSET)) as *const u32).read_unaligned()
    };
    if source_flags & STATUS_MASK != REQUIRED_STATUS {
        return 0;
    }

    let target_state_view = target_state.wrapping_add(TARGET_STATE_OFFSET);
    let target_needs_apply = lf_checker_rt::callee_thiscall!(
        TARGET_STATE_QUERY,
        u8,
        target_state_view,
        0x772u32,
        0u32
    );
    if target_needs_apply == 0 {
        let _apply_result = lf_checker_rt::callee_thiscall!(
            APPLY_MEDIC_TASK,
            u32,
            target_state,
            source_task,
            1u32
        );
    }

    let manager = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_MANAGER_VA).read_unaligned()
    };
    let allocated_task = lf_checker_rt::callee_thiscall!(
        TASK_POOL_ALLOCATOR,
        u32,
        manager
    );
    let result_task = if allocated_task == 0 {
        0
    } else {
        lf_checker_rt::callee_thiscall!(
            MEDIC_TASK_CONSTRUCTOR,
            u32,
            allocated_task,
            source_task,
            0u32
        )
    };

    let should_mark_medic = lf_checker_rt::callee_thiscall!(
        TARGET_STATE_QUERY,
        u8,
        target_state_view,
        0x770u32,
        0u32
    );
    if should_mark_medic != 0 {
        unsafe {
            ((result_task.wrapping_add(RESULT_FLAGS_OFFSET)) as *mut u32)
                .write_unaligned(
                    ((result_task.wrapping_add(RESULT_FLAGS_OFFSET)) as *const u32)
                        .read_unaligned()
                        | MEDIC_FLAG,
                );
        }
    }

    result_task
});
