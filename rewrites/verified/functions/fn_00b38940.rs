// original: 0x00B38940 ped_task_transition_updates

/// Apply the task-related updates selected by three byte flags.
///
/// `ped` points to an object whose task state pointer is at `+0x21c` and
/// whose update targets are at `+0x224`, `+0x2b0`, and `+0x26c`. The task
/// state holds a signed-agnostic 32-bit task index at `+0x12c`. When
/// `query` is nonzero, a task query must pass before the table path is
/// considered. If the global phase word equals three, two additional
/// queries can provide a task directly; a zero result falls back to the
/// table lookup. A nonzero task result is sent to the task update method.
/// When `initialize` is nonzero, the state selects
/// one of two exact pairs of single-precision constants for a callback.
/// When `refresh` is nonzero, two final update calls run in order.
///
/// The three flags are interpreted by their low byte. Calls retain the original order
/// and calling conventions. All object offsets and data addresses below are
/// file addresses or byte offsets passed through the checker runtime.
lf_checker_rt::export!(cdecl, rw_00b38940(
    ped: u32,
    query: u32,
    initialize: u32,
    refresh: u32,
) -> () {
    const PED_TASK_STATE: u32 = 0x21c;
    const PED_TASK_TABLE_TARGET: u32 = 0x224;
    const PED_TASK_UPDATE_TARGET: u32 = 0x2b0;
    const PED_FLAGS: u32 = 0x26c;
    const STATE_TASK_INDEX: u32 = 0x12c;
    const STATE_TASK_DATA: u32 = 0x80;
    const TASK_ENTRY_STRIDE: u32 = 20;
    const TASK_TABLE_FILE_VA: u32 = 0x0171_f994;
    const GAME_PHASE_FILE_VA: u32 = 0x011d_6fd4;
    const BLEND_HIGH_FILE_VA: u32 = 0x00fe_881c;
    const BLEND_LOW_FILE_VA: u32 = 0x00fe_87e4;
    const BLEND_ONE_FILE_VA: u32 = 0x00fe_88e8;
    const ENQUEUE_DELAY: u32 = 0x61a8;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let task_state = read_u32(ped.wrapping_add(PED_TASK_STATE));
        let task_index = read_u32(task_state.wrapping_add(STATE_TASK_INDEX));

        if (query as u8) != 0 {
            let task_is_available =
                lf_checker_rt::callee_cdecl!(1, u8, task_index) != 0;

            if task_is_available {
                let game_phase =
                    lf_checker_rt::global::<u32>(GAME_PHASE_FILE_VA).read_unaligned();
                let phase_task = if game_phase == 3 {
                    let phase_gate = lf_checker_rt::callee_cdecl!(2, u8,) != 0;
                    if phase_gate {
                        lf_checker_rt::callee_cdecl!(3, u32, task_index)
                    } else {
                        0
                    }
                } else {
                    0
                };

                let found_task = if phase_task != 0 {
                    phase_task
                } else {
                    let task_flags = read_u32(ped.wrapping_add(PED_FLAGS));
                    let task_kind = (task_flags >> 2) & 1;
                    let table_base =
                        lf_checker_rt::global::<u8>(TASK_TABLE_FILE_VA) as u32;
                    let table_entry = table_base
                        .wrapping_add(task_index.wrapping_mul(TASK_ENTRY_STRIDE));
                    lf_checker_rt::callee_thiscall!(4, u32, table_entry, task_kind)
                };

                if found_task != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(
                        5,
                        u32,
                        ped.wrapping_add(PED_TASK_UPDATE_TARGET),
                        found_task,
                        ENQUEUE_DELAY,
                        1,
                        0,
                        0,
                    );
                }
            }
        }

        if (initialize as u8) != 0 {
            let state_kind = read_u32(task_state.wrapping_add(STATE_TASK_INDEX));
            let blend_high =
                lf_checker_rt::global::<f32>(BLEND_HIGH_FILE_VA).read_unaligned();
            let blend_low =
                lf_checker_rt::global::<f32>(BLEND_LOW_FILE_VA).read_unaligned();
            let blend_one =
                lf_checker_rt::global::<f32>(BLEND_ONE_FILE_VA).read_unaligned();

            let (first_weight, second_weight, callback_kind) = if state_kind == 2 {
                (blend_one, blend_high, 2)
            } else {
                (blend_high, blend_low, 0)
            };
            let task_data = read_u32(ped.wrapping_add(PED_TASK_STATE));

            let _ = lf_checker_rt::callee_cdecl!(
                6,
                u32,
                ped,
                task_data.wrapping_add(STATE_TASK_DATA),
                task_data,
                first_weight.to_bits(),
                second_weight.to_bits(),
                callback_kind,
                0,
            );
        }

        if (refresh as u8) != 0 {
            let target = read_u32(ped.wrapping_add(PED_TASK_TABLE_TARGET));
            let _ = lf_checker_rt::callee_thiscall!(7, u32, target);
            let _ = lf_checker_rt::callee_cdecl!(8, u32, ped);
        }
    }
});
