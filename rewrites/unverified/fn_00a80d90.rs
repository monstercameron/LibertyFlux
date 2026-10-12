// original: 0x00a80d90 CScenarioTaskInfo::ensure_target_entity

/// Ensure the scenario target entity is present and clear its stale status.
///
/// This `thiscall` method acts only when byte `+0x24` is nonzero and target
/// pointer `+0x1C` is null. A zero 16-bit target identifier at `+0x40` builds
/// a helper payload from the three single-precision words at `+0x30`, `+0x34`
/// and `+0x38`, plus a fixed fourth parameter, then asks the helper to populate
/// the target field. A nonzero identifier performs a lookup; a non-null result
/// is queried through vtable slot one, and a second nonzero query stores its
/// result at `+0x1C`. A resulting target is finalized through another helper.
/// If the target field remains null while `+0x24` is nonzero, word `+0x20` is
/// cleared. The helper calls use checker stand-ins. The stack payload's fourth
/// dword is not written by the original; the contract fills that word with zero
/// and snapshots all five payload words at the call.
lf_checker_rt::export!(thiscall, rw_00a80d90(task_info: u32) -> () {
    const TARGET_POINTER_OFFSET: u32 = 0x1C;
    const ACTIVE_FLAG_OFFSET: u32 = 0x24;
    const TARGET_ID_OFFSET: u32 = 0x40;
    const POSITION_X_OFFSET: u32 = 0x30;
    const POSITION_Y_OFFSET: u32 = 0x34;
    const POSITION_Z_OFFSET: u32 = 0x38;
    const CLEARED_STATUS_OFFSET: u32 = 0x20;
    const TARGET_POINTER_FIELD: u32 = 0x1C;
    const TARGET_SINGLETON_VA: u32 = 0x01908EF0;
    const TARGET_CREATION_CALLBACK_VA: u32 = 0x00A80E50;
    const TARGET_LOOKUP: u32 = 1;
    const TARGET_QUERY_VIRTUAL: u32 = 2;
    const BUILD_TARGET: u32 = 3;
    const FINALIZE_TARGET: u32 = 4;
    const PAYLOAD_FIXED_FLOAT_BITS: u32 = 0x41200000;

    let active = unsafe {
        ((task_info.wrapping_add(ACTIVE_FLAG_OFFSET)) as *const u8).read()
    } != 0;

    if active {
        let initial_target = unsafe {
            ((task_info.wrapping_add(TARGET_POINTER_OFFSET)) as *const u32)
                .read_unaligned()
        };
        if initial_target == 0 {
            let target_id = unsafe {
                ((task_info.wrapping_add(TARGET_ID_OFFSET)) as *const u16).read_unaligned()
            };
            if target_id == 0 {
                let mut payload = [0u32; 5];
                payload[0] = unsafe {
                    ((task_info.wrapping_add(POSITION_X_OFFSET)) as *const u32)
                        .read_unaligned()
                };
                payload[1] = unsafe {
                    ((task_info.wrapping_add(POSITION_Y_OFFSET)) as *const u32)
                        .read_unaligned()
                };
                payload[2] = unsafe {
                    ((task_info.wrapping_add(POSITION_Z_OFFSET)) as *const u32)
                        .read_unaligned()
                };
                payload[4] = PAYLOAD_FIXED_FLOAT_BITS;
                let payload_pointer = payload.as_mut_ptr() as u32;
                let callback = lf_checker_rt::relocated(TARGET_CREATION_CALLBACK_VA);
                let _build_result = lf_checker_rt::callee_cdecl!(
                    BUILD_TARGET,
                    u32,
                    payload_pointer,
                    callback,
                    task_info,
                    0x32u32,
                    0x0Cu32
                );
            } else {
                let target_list = lf_checker_rt::relocated(TARGET_SINGLETON_VA);
                let candidate = lf_checker_rt::callee_thiscall!(
                    TARGET_LOOKUP,
                    u32,
                    target_list,
                    u32::from(target_id),
                    0u32
                );
                if candidate != 0 {
                    let has_target = lf_checker_rt::callee_thiscall!(
                        TARGET_QUERY_VIRTUAL,
                        u32,
                        candidate
                    );
                    if has_target != 0 {
                        let resolved_target = lf_checker_rt::callee_thiscall!(
                            TARGET_QUERY_VIRTUAL,
                            u32,
                            candidate
                        );
                        unsafe {
                            ((task_info.wrapping_add(TARGET_POINTER_OFFSET)) as *mut u32)
                                .write_unaligned(resolved_target);
                        }
                    }
                }
            }

            let target = unsafe {
                ((task_info.wrapping_add(TARGET_POINTER_OFFSET)) as *const u32)
                    .read_unaligned()
            };
            if target != 0 {
                let field = task_info.wrapping_add(TARGET_POINTER_FIELD);
                let _finalize_result = lf_checker_rt::callee_thiscall!(
                    FINALIZE_TARGET,
                    u32,
                    target,
                    field
                );
            }
        }
    }

    let target_at_exit = unsafe {
        ((task_info.wrapping_add(TARGET_POINTER_OFFSET)) as *const u32)
            .read_unaligned()
    };
    if target_at_exit == 0 && active {
        unsafe {
            ((task_info.wrapping_add(CLEARED_STATUS_OFFSET)) as *mut u32)
                .write_unaligned(0);
        }
    }
});
