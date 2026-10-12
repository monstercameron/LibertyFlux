// original: 0x00DC1760 peds_task_path_solve

/// Advances one pedestrian path task against two world positions.
///
/// `ped` is the owning entity and `task` is the task state passed on the
/// stack. The task stores flags at `+0x14`, its two position records at
/// `+0x40` and `+0x50`, a path count at `+0x1c`, and path points at `+0x80`.
/// The routine clears the task's transient vectors, publishes the selected
/// frame and positions to shared engine state, resolves both positions to
/// navigation cells, and asks the navigation helpers to validate and build
/// a path. A missing cell records status `-4`; a gated missing second node
/// records `-14` when the global recovery flag is set. The marker check runs
/// only with two usable navigation nodes; a successful safe-position lookup
/// supplies the second node when the direct lookup misses. Every non-early-
/// return path advances the first component of each point by the shared step
/// value. The semantic
/// result is returned in AL; upper EAX bits are not part of this function's
/// result.
///
/// All helper calls below are intercepted by the checker contract. Pointers
/// in the task and entity retain the original 32-bit game layout.
lf_checker_rt::export!(thiscall, rw_00dc1760(ped: u32, task: u32) -> u32 {
    unsafe {
        const TASK_FLAGS: u32 = 0x14;
        const TASK_STATUS: u32 = 0x08;
        const TASK_STATE: u32 = 0x10;
        const TASK_PATH_COUNT: u32 = 0x1c;
        const TASK_FIRST_POINT: u32 = 0x40;
        const TASK_SECOND_POINT: u32 = 0x50;
        const TASK_POINTS: u32 = 0x80;
        const TASK_POINT_STEP_FIELDS: [u32; 8] = [
            0x1e4, 0x1ec, 0x1f4, 0x1fc, 0x204, 0x20c, 0x214, 0x21c,
        ];
        const TASK_TRANSIENT_VALUE: u32 = 0x224;
        const TASK_CELL_OVERRIDE: u32 = 0x334;
        const TASK_WEAPON_ROW: u32 = 0x344;
        const TASK_WEAPON_DATA: u32 = 0x350;
        const PED_COMPONENT: u32 = 0xc18;
        const PED_WEAPON_ROWS: u32 = 0xc54;
        const COMPONENT_FLAGS: u32 = 0x14;
        const NAV_NODE_FLAGS: u32 = 0x50;
        const NAV_REC_VECTOR: u32 = 0x70;
        const NAV_VECTOR_OFFSET: u32 = 0x10;
        const NAV_ROW_STRIDE: u32 = 0x280;
        const NAV_ROW_WEAPON_FLAGS: u32 = 0x26c;
        const PATH_POINT_FLAGS: u32 = 0x19c;
        const PATH_POINT_COUNT_LIMIT: u32 = 16;
        const INVALID_CELL: u32 = 0x0fff;
        const WEAPON_ROW_LIMIT: u32 = 0x200;
        const FRAME_FLAG: u32 = 0x0040_0000;
        const SKIP_ENTITY_RELEASES: u32 = 0x0000_2000;
        const USE_CELL_OVERRIDE: u32 = 0x0100_0000;
        const SAFE_POSITION: u32 = 0x0002_0000;
        const SKIP_NORMALIZE_AND_REFINE: u32 = 0x0200_0000;
        const SMOOTH_PATH: u32 = 0x0010_0000;
        const REFINE_LIST: u32 = 0x0008_0000;
        const WEAPON_BIT: u32 = 0x0000_0200;
        const GLOBAL_CLOCK_BASE: u32 = 0x016b_6b28;
        const GLOBAL_COMPONENT_STATE: u32 = 0x016b_6b24;
        const GLOBAL_TASK_FLAG: u32 = 0x0179_fd49;
        const GLOBAL_FRAME_RANGE: u32 = 0x0179_fd4c;
        const GLOBAL_POSITION_A: u32 = 0x017a_1db0;
        const GLOBAL_POSITION_B: u32 = 0x017a_1dc0;
        const GLOBAL_NAV_RECOVERY: u32 = 0x016b_7bf1;
        const GLOBAL_PATH_ENABLE: u32 = 0x0104_8267;
        const GLOBAL_PATH_MODE: u32 = 0x0104_827c;
        const GLOBAL_REFINE_ENABLE: u32 = 0x017a_66f8;
        const GLOBAL_TASK_DISPATCH: u32 = 0x016b_6b1d;
        const FRAME_OFFSET: u32 = 0x00fe_87e4;
        const POINT_STEP: u32 = 0x00fe_8830;
        const PATH_SPEED: u32 = 0x4248_0000;
        const SAFE_PATH_SPEED: u32 = 0x44fa_0000;
        const CALL_CLOCK: u32 = 1;
        const CALL_RELEASE_FRAGMENTS: u32 = 2;
        const CALL_RELEASE_GUARD: u32 = 3;
        const CALL_APPLY_DROP: u32 = 4;
        const CALL_WORLD_TO_CELL: u32 = 5;
        const CALL_FIND_CELL: u32 = 6;
        const CALL_FIRE_PED_WEAPON: u32 = 7;
        const CALL_FIND_NEAREST: u32 = 8;
        const CALL_SAFE_POSITION: u32 = 9;
        const CALL_VALIDATE: u32 = 10;
        const CALL_BUILD_PATH: u32 = 11;
        const CALL_NORMALIZE: u32 = 12;
        const CALL_SMOOTH: u32 = 13;
        const CALL_REFINE: u32 = 14;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn global_u32(file_va: u32) -> u32 {
            unsafe { read_u32(lf_checker_rt::global::<u32>(file_va) as u32) }
        }
        #[inline(always)]
        unsafe fn set_global_u32(file_va: u32, value: u32) {
            unsafe { write_u32(lf_checker_rt::global::<u32>(file_va) as u32, value) }
        }
        #[inline(always)]
        unsafe fn global_f32(file_va: u32) -> f32 {
            f32::from_bits(unsafe { global_u32(file_va) })
        }
        #[inline(always)]
        unsafe fn set_global_f32(file_va: u32, value: f32) {
            unsafe { set_global_u32(file_va, value.to_bits()) }
        }
        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        // Clear the task's eight transient 64-bit vector records.
        for offset in TASK_POINT_STEP_FIELDS {
            write_u32(task.wrapping_add(offset), 0);
            write_u32(task.wrapping_add(offset + 4), 0);
        }
        write_u32(task.wrapping_add(TASK_TRANSIENT_VALUE), 0);
        let task_flags = read_u32(task.wrapping_add(TASK_FLAGS));

        // The timer helper advances state on the owning entity. The frame
        // value is then published to all four shared range slots.
        let _ = lf_checker_rt::callee_thiscall!(CALL_CLOCK, u32, ped);
        let mut frame_value = global_f32(GLOBAL_CLOCK_BASE);
        if task_flags & FRAME_FLAG != 0 {
            frame_value = add_f32(frame_value, global_f32(FRAME_OFFSET));
        }
        for offset in [0u32, 4, 8, 12] {
            set_global_f32(GLOBAL_FRAME_RANGE.wrapping_add(offset), frame_value);
        }

        if task_flags & SKIP_ENTITY_RELEASES == 0 {
            let _ = lf_checker_rt::callee_thiscall!(CALL_RELEASE_FRAGMENTS, u32, ped);
            let _ = lf_checker_rt::callee_thiscall!(CALL_RELEASE_GUARD, u32, ped);
        }

        let component = read_u32(ped.wrapping_add(PED_COMPONENT));
        let component_flags = read_u32(component.wrapping_add(COMPONENT_FLAGS));
        let state_address = lf_checker_rt::global::<u8>(GLOBAL_COMPONENT_STATE) as u32;
        let state_before = read_u8(state_address);
        let state_after = if component_flags & 0x1000_0000 != 0 { 1 } else { state_before };
        write_u8(state_address, state_after);
        write_u8(
            lf_checker_rt::global::<u8>(GLOBAL_TASK_FLAG) as u32,
            ((task_flags >> 7) & 1) as u8,
        );

        // Copy both target positions into the task's working records and
        // publish those values for the navigation routines.
        for (source, destination) in [
            (0x20u32, TASK_FIRST_POINT),
            (0x24, TASK_FIRST_POINT + 4),
            (0x28, TASK_FIRST_POINT + 8),
            (0x2c, TASK_FIRST_POINT + 12),
            (0x30, TASK_SECOND_POINT),
            (0x34, TASK_SECOND_POINT + 4),
            (0x38, TASK_SECOND_POINT + 8),
            (0x3c, TASK_SECOND_POINT + 12),
        ] {
            write_u32(
                task.wrapping_add(destination),
                read_u32(task.wrapping_add(source)),
            );
        }
        for index in 0..4u32 {
            set_global_u32(
                GLOBAL_POSITION_A.wrapping_add(index * 4),
                read_u32(task.wrapping_add(TASK_FIRST_POINT + index * 4)),
            );
            set_global_u32(
                GLOBAL_POSITION_B.wrapping_add(index * 4),
                read_u32(task.wrapping_add(TASK_SECOND_POINT + index * 4)),
            );
        }

        // Resolve the two points, or reuse the task's explicit first cell.
        let (first_cell, second_cell) = if task_flags & USE_CELL_OVERRIDE != 0
            && read_u32(task.wrapping_add(TASK_CELL_OVERRIDE)) != INVALID_CELL
        {
            let cell = read_u32(task.wrapping_add(TASK_CELL_OVERRIDE));
            (cell, cell)
        } else {
            if task_flags & SKIP_ENTITY_RELEASES == 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    CALL_APPLY_DROP,
                    u32,
                    ped,
                    task.wrapping_add(TASK_FIRST_POINT),
                    1,
                    0,
                    0,
                );
            }
            let first = lf_checker_rt::callee_cdecl!(
                CALL_WORLD_TO_CELL,
                u32,
                task.wrapping_add(TASK_FIRST_POINT),
            );
            if first == INVALID_CELL {
                let state = read_u32(task.wrapping_add(TASK_STATE));
                write_u32(task.wrapping_add(TASK_STATE), (state & !2) | 4);
                write_u32(task.wrapping_add(TASK_STATUS), u32::MAX - 3);
                write_u32(task.wrapping_add(TASK_PATH_COUNT), 0);
                return 0;
            }

            if task_flags & SKIP_ENTITY_RELEASES == 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    CALL_APPLY_DROP,
                    u32,
                    ped,
                    task.wrapping_add(TASK_SECOND_POINT),
                    0,
                    1,
                    0,
                );
            }
            let second = lf_checker_rt::callee_cdecl!(
                CALL_WORLD_TO_CELL,
                u32,
                task.wrapping_add(TASK_SECOND_POINT),
            );
            if second == INVALID_CELL {
                let state = read_u32(task.wrapping_add(TASK_STATE));
                write_u32(task.wrapping_add(TASK_STATE), (state & !2) | 4);
                write_u32(task.wrapping_add(TASK_STATUS), u32::MAX - 3);
                write_u32(task.wrapping_add(TASK_PATH_COUNT), 0);
                return 0;
            }
            (first, second)
        };

        let first_node = lf_checker_rt::callee_cdecl!(CALL_FIND_CELL, u32, first_cell);
        let mut second_node = lf_checker_rt::callee_cdecl!(CALL_FIND_CELL, u32, second_cell);

        // Clear the matching weapon row when the first navigation node asks
        // for a fire response. The game stores each row 0x280 bytes apart.
        if first_node != 0 && read_u8(first_node.wrapping_add(NAV_NODE_FLAGS)) & 4 != 0 {
            let weapon_row = read_u16(task.wrapping_add(TASK_WEAPON_ROW)) as u32;
            if weapon_row < WEAPON_ROW_LIMIT {
                let rows = read_u32(ped.wrapping_add(PED_WEAPON_ROWS));
                let word = rows
                    .wrapping_add(weapon_row.wrapping_mul(NAV_ROW_STRIDE))
                    .wrapping_add(NAV_ROW_WEAPON_FLAGS);
                write_u32(word, read_u32(word) & !2);
            }
            let _ = lf_checker_rt::callee_thiscall!(
                CALL_FIRE_PED_WEAPON,
                u32,
                first_node,
                task.wrapping_add(TASK_WEAPON_DATA),
            );
        }

        // A missing second node takes the guarded safe-position path. Its
        // output record is local to this call; the contract models that
        // callee and its two frame pointers explicitly.
        if task_flags & SAFE_POSITION != 0 && second_node == 0 {
            if read_u8(lf_checker_rt::global::<u8>(GLOBAL_NAV_RECOVERY) as u32) != 0 {
                write_u32(task.wrapping_add(TASK_STATUS), u32::MAX - 13);
                return 0;
            }
            let nearest = lf_checker_rt::callee_cdecl!(
                CALL_FIND_NEAREST,
                u32,
                task.wrapping_add(TASK_SECOND_POINT),
            );
            let nav_vector = read_u32(nearest.wrapping_add(NAV_REC_VECTOR));
            let scale = global_f32(POINT_STEP);
            let x = mul_f32(
                add_f32(
                    f32::from_bits(read_u32(nav_vector)),
                    f32::from_bits(read_u32(nav_vector.wrapping_add(NAV_VECTOR_OFFSET))),
                ),
                scale,
            );
            let y = mul_f32(
                add_f32(
                    f32::from_bits(read_u32(nav_vector.wrapping_add(4))),
                    f32::from_bits(read_u32(nav_vector.wrapping_add(NAV_VECTOR_OFFSET + 4))),
                ),
                scale,
            );
            let mut requested_position = [x.to_bits(), y.to_bits(), read_u32(task.wrapping_add(TASK_SECOND_POINT + 8)), 0];
            let mut safe_output = [first_node, 0, 0, 0];
            let mode = if task_flags & SKIP_ENTITY_RELEASES == 0 { 0x12 } else { 0x10 };
            let safe = lf_checker_rt::callee_cdecl!(
                CALL_SAFE_POSITION,
                u32,
                requested_position.as_mut_ptr() as u32,
                safe_output.as_mut_ptr() as u32,
                PATH_SPEED,
                mode,
                0,
                0,
                0,
            );
            if safe != 0 {
                second_node = nearest;
                for (index, value) in safe_output.iter().copied().enumerate() {
                    write_u32(task.wrapping_add(TASK_SECOND_POINT + index as u32 * 4), value);
                }
                if task_flags & SKIP_ENTITY_RELEASES == 0 {
                    let applied = lf_checker_rt::callee_thiscall!(
                        CALL_APPLY_DROP,
                        u32,
                        ped,
                        task.wrapping_add(TASK_SECOND_POINT),
                        0,
                        0,
                        0,
                    ) as u8;
                    if applied == 0 {
                        let _ = lf_checker_rt::callee_thiscall!(
                            CALL_APPLY_DROP,
                            u32,
                            ped,
                            task.wrapping_add(TASK_SECOND_POINT),
                            0,
                            u32::from(task_flags & WEAPON_BIT != 0),
                            0,
                        );
                    }
                }
                write_u32(task.wrapping_add(TASK_SECOND_POINT + 16), SAFE_PATH_SPEED);
            }
        }

        // Validate and build only when both cell lookups produced records.
        if first_node != 0 && second_node != 0 {
            write_u32(ped.wrapping_add(PED_COMPONENT), task);
            write_u8(
                lf_checker_rt::global::<u8>(GLOBAL_TASK_DISPATCH) as u32,
                ((task_flags >> 18) & 1) as u8,
            );
            let saved_path_mode = global_u32(GLOBAL_PATH_MODE);
            if task_flags & SKIP_ENTITY_RELEASES != 0 {
                set_global_u32(GLOBAL_PATH_MODE, 0);
            }
            let valid = lf_checker_rt::callee_thiscall!(
                CALL_VALIDATE,
                u32,
                ped,
                first_node,
                second_node,
                task,
            ) as u8;
            if valid == 0 {
                set_global_u32(GLOBAL_PATH_MODE, saved_path_mode);
                return 0;
            }
            let built = lf_checker_rt::callee_thiscall!(
                CALL_BUILD_PATH,
                u32,
                ped,
                task,
                task.wrapping_add(TASK_FIRST_POINT),
                task.wrapping_add(TASK_SECOND_POINT),
                PATH_POINT_COUNT_LIMIT,
            ) as u8;
            if built == 0 {
                set_global_u32(GLOBAL_PATH_MODE, saved_path_mode);
                return 0;
            }
            if read_u8(lf_checker_rt::global::<u8>(GLOBAL_PATH_ENABLE) as u32) != 0
                && task_flags & SKIP_NORMALIZE_AND_REFINE == 0
            {
                let _ = lf_checker_rt::callee_thiscall!(CALL_NORMALIZE, u32, ped, task);
            }
            if task_flags & SMOOTH_PATH != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    CALL_SMOOTH,
                    u32,
                    ped,
                    task,
                    task.wrapping_add(TASK_FIRST_POINT),
                    task.wrapping_add(TASK_SECOND_POINT),
                    PATH_POINT_COUNT_LIMIT,
                );
            }
            if task_flags & REFINE_LIST != 0
                && read_u8(lf_checker_rt::global::<u8>(GLOBAL_REFINE_ENABLE) as u32) == 0
                && task_flags & SKIP_NORMALIZE_AND_REFINE == 0
            {
                let _ = lf_checker_rt::callee_thiscall!(CALL_REFINE, u32, ped, task);
            }
            set_global_u32(GLOBAL_PATH_MODE, saved_path_mode);
        }

        // Mark the current point only after both navigation nodes resolved.
        let point_count = read_u32(task.wrapping_add(TASK_PATH_COUNT));
        if first_node != 0 && second_node != 0 && point_count < PATH_POINT_COUNT_LIMIT {
            let point = task
                .wrapping_add(TASK_POINTS)
                .wrapping_add(point_count.wrapping_mul(16));
            let delta_x = sub_f32(
                f32::from_bits(read_u32(point.wrapping_add(4))),
                global_f32(GLOBAL_POSITION_B + 4),
            );
            let delta_y = sub_f32(
                f32::from_bits(read_u32(point)),
                global_f32(GLOBAL_POSITION_B),
            );
            let distance_squared = add_f32(
                mul_f32(delta_x, delta_x),
                mul_f32(delta_y, delta_y),
            );
            if distance_squared > global_f32(FRAME_OFFSET) {
                let marker = task
                    .wrapping_add(PATH_POINT_FLAGS)
                    .wrapping_add(point_count.wrapping_mul(4));
                write_u16(marker, read_u16(marker) | 0x80);
            }
        }
        let step = global_f32(POINT_STEP);
        let mut index = 0u32;
        while index < point_count {
            let coordinate = task
                .wrapping_add(0x98)
                .wrapping_add(index.wrapping_mul(16));
            let old = f32::from_bits(read_u32(coordinate));
            write_u32(coordinate, add_f32(old, step).to_bits());
            index += 1;
        }
        1
    }
});
