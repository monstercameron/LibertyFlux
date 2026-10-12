// original: 0x00B30950 ped_task_runnable_check

/// Decide whether a task record can run for the supplied pedestrian state.
///
/// The cdecl argument points to a task record. Its unsigned kind is at `+0`,
/// position at `+0x10..+0x18`, and candidate task at `+0x28`. Kinds above 25
/// are accepted. Kinds 3 through 11 require candidate flags `0xc0` under mask
/// `0x3c0`, a clear abort bit at `+0x26c`, and a child state other than 2 at
/// `+0x12c`. Kinds 12 and 22 require candidate flags `0x80` under the same
/// mask, then consult the target's virtual slot `+0x128`; a nonzero byte
/// result accepts immediately. Otherwise the task position is compared with
/// the current player position, accepting a squared distance at most 15.
/// The original's COMISS/JBE pair also accepts an unordered distance (NaN).
/// Every other in-range kind is accepted. The function returns its decision
/// in AL and leaves cdecl stack cleanup to its caller.
lf_checker_rt::export!(cdecl, rw_00b30950(ped_record: u32) -> u8 {
    unsafe {
        const PED_KIND: u32 = 0x00;
        const PED_POSITION_X: u32 = 0x10;
        const PED_POSITION_Y: u32 = 0x14;
        const PED_POSITION_Z: u32 = 0x18;
        const PED_CANDIDATE_TASK: u32 = 0x28;
        const TASK_FLAGS: u32 = 0x28;
        const TASK_ABORT_FLAGS: u32 = 0x26c;
        const TASK_CHILD_STATE: u32 = 0x21c;
        const TASK_EVENT_TARGET: u32 = 0xf50;
        const CHILD_STATE: u32 = 0x12c;
        const TASK_VTABLE: u32 = 0x00;
        const VTABLE_RUN_CHECK: u32 = 0x128;
        const POSITION_COMPONENTS: u32 = 3;
        const DISTANCE_LIMIT_GLOBAL_VA: u32 = 0x00fe8b20;
        const DISTANCE_LIMIT: f32 = 15.0;
        const RUNNABLE_KIND_MIN: u32 = 3;
        const RUNNABLE_KIND_MAX: u32 = 11;
        const EVENT_KIND_A: u32 = 12;
        const EVENT_KIND_B: u32 = 22;
        const FLAGS_MASK: u32 = 0x3c0;
        const ORDINARY_TASK_FLAGS: u32 = 0xc0;
        const EVENT_TASK_FLAGS: u32 = 0x80;
        const ABORT_BIT: u8 = 0x04;
        const TERMINAL_CHILD_STATE: u32 = 2;
        const RUN_CHECK_CALLEE: u32 = 1;
        const PLAYER_POSITION_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }

        #[inline(always)]
        fn ordered_sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn ordered_mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        #[inline(always)]
        fn ordered_add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }

        #[inline(always)]
        unsafe fn within_player_range(ped_record: u32) -> u8 {
            unsafe {
                let mut requested_position = [0.0f32; POSITION_COMPONENTS as usize];
                let current_position = lf_checker_rt::callee_cdecl!(
                    PLAYER_POSITION_CALLEE,
                    u32,
                    requested_position.as_mut_ptr() as u32,
                );
                let x_delta = ordered_sub(
                    read_f32(ped_record + PED_POSITION_X),
                    read_f32(current_position),
                );
                let y_delta = ordered_sub(
                    read_f32(ped_record + PED_POSITION_Y),
                    read_f32(current_position + 4),
                );
                let z_delta = ordered_sub(
                    read_f32(ped_record + PED_POSITION_Z),
                    read_f32(current_position + 8),
                );
                let y_squared = ordered_mul(y_delta, y_delta);
                let x_squared = ordered_mul(x_delta, x_delta);
                let z_squared = ordered_mul(z_delta, z_delta);
                let xy_squared = ordered_add(y_squared, x_squared);
                let distance_squared = ordered_add(xy_squared, z_squared);
                let max_distance_squared =
                    lf_checker_rt::global::<f32>(DISTANCE_LIMIT_GLOBAL_VA).read();
                let _documented_limit = DISTANCE_LIMIT;
                u8::from(distance_squared.is_nan() || distance_squared <= max_distance_squared)
            }
        }

        let kind = read_u32(ped_record + PED_KIND);
        if kind > 25 {
            return 1;
        }

        match kind {
            RUNNABLE_KIND_MIN..=RUNNABLE_KIND_MAX => {
                let candidate = read_u32(ped_record + PED_CANDIDATE_TASK);
                if candidate == 0 {
                    return 0;
                }
                if read_u32(candidate + TASK_FLAGS) & FLAGS_MASK != ORDINARY_TASK_FLAGS {
                    return 0;
                }
                if read_u8(candidate + TASK_ABORT_FLAGS) & ABORT_BIT != 0 {
                    return 0;
                }
                let child = read_u32(candidate + TASK_CHILD_STATE);
                u8::from(read_u32(child + CHILD_STATE) != TERMINAL_CHILD_STATE)
            }
            EVENT_KIND_A | EVENT_KIND_B => {
                let candidate = read_u32(ped_record + PED_CANDIDATE_TASK);
                if candidate == 0 {
                    return 0;
                }
                if read_u32(candidate + TASK_FLAGS) & FLAGS_MASK != EVENT_TASK_FLAGS {
                    return 0;
                }
                let target = read_u32(candidate + TASK_EVENT_TARGET);
                if target == 0 {
                    return 0;
                }
                let vtable = read_u32(target + TASK_VTABLE);
                let run_check = read_u32(vtable + VTABLE_RUN_CHECK);
                let run_check: extern "thiscall" fn(u32) -> u8 =
                    core::mem::transmute(run_check as usize);
                if run_check(target) != 0 {
                    return 1;
                }
                within_player_range(ped_record)
            }
            _ => 1,
        }
    }
});
