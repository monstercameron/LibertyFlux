// original: 0x00CCFD10 ped_task_advance_runtime_state

/// Advance a ped task's time state or classify its current tracking state.
/// `task` is the task object in ECX and `owner` is the single stack argument.
/// The routine first fetches the task's resource, then clears the task's
/// reinitialization bit and returns status 3 when that bit was set. Otherwise
/// it asks the task and owner helpers for state, advances the timer at byte
/// offset `TASK_TIMER` on the ordinary path, or compares a three component
/// position delta against two radius globals on the tracking path. Its result
/// is a status code: 0 for no transition, 1 for the outer radius / flag case,
/// 2 for the inner radius case, and 3 for completion or a cleared timer path.
///
/// The ordinary timer path selects one of two global durations, multiplies it
/// by `TASK_TIME_SCALE`, and adds that increment to the existing timer in that
/// operand order. Ordered comparisons preserve the original COMISS branch
/// behavior for unordered values. The tracking path follows a linked resource
/// list and may ask helpers to transform a stack local vector and compare it
/// against two tracker vectors. Those temporary vector arguments are passed
/// to intercepted callees; the proof contract records snapshots when it
/// exercises those calls. The checker contract currently proves only the
/// reinitialization-bit path, so the other decision branches remain outside
/// this result's tested input set.
lf_checker_rt::export!(thiscall, rw_00ccfd10(task: u32, owner: u32) -> u32 {
    const TASK_ARGUMENT: u32 = 0x08;
    const TASK_TIMER: u32 = 0x34;
    const TASK_TARGET: u32 = 0x50;
    const TASK_FLAGS: u32 = 0xF0;
    const TASK_FLAGS_HIGH: u32 = 0xF1;
    const OWNER_ROUTE_A: u32 = 0x218;
    const OWNER_ROUTE_B: u32 = 0x219;
    const OWNER_MANAGER: u32 = 0x2B0;
    const OWNER_POSITION: u32 = 0x20;
    const RESOURCE_FLAGS: u32 = 0x28;
    const RESOURCE_EXTRA_FLAGS: u32 = 0x26C;
    const RESOURCE_FORCED_STATUS: u32 = 0xB30;
    const RESOURCE_DATA: u32 = 0x224;
    const RESOURCE_TRACKER: u32 = 0x2E0;
    const TRACK_NEXT: u32 = 0x0C;
    const TRACK_KIND: u32 = 0x04;
    const TRACK_RANK: u32 = 0x08;
    const FORCE_REINITIALIZE: u8 = 0x10;
    const HAS_TARGET: u8 = 0x40;
    const USE_INNER_RADIUS: u8 = 0x80;
    const USE_OUTER_RADIUS: u8 = 0x04;
    const TRACK_KIND_SPECIAL: u32 = 0x2DE;
    const POSITION_X: u32 = 0x30;
    const LOOKUP_CALLEE: u32 = 1;
    const TRANSITION_CALLEE: u32 = 2;
    const OWNER_ROUTE_CALLEE: u32 = 3;
    const OWNER_STATE_CALLEE: u32 = 4;
    const RESOURCE_STATE_CALLEE: u32 = 5;
    const TRANSFORM_VECTOR_CALLEE: u32 = 6;
    const FIND_TRACKER_CALLEE: u32 = 7;
    const COMPARE_TRACKER_CALLEE: u32 = 8;
    const INNER_RADIUS_CALLEE: u32 = 9;
    const LOOKUP_TABLE_A: u32 = 0x010519C0;
    const LOOKUP_TABLE_B: u32 = 0x01112778;
    const TRANSITION_DURATION: u32 = 0x00FE8A24;
    const IDLE_DURATION: u32 = 0x00FE88E8;
    const TASK_TIME_SCALE: u32 = 0x011735BC;
    const TIMER_THRESHOLD: u32 = 0x01051994;
    const INNER_RADIUS: u32 = 0x01051940;
    const OUTER_RADIUS: u32 = 0x01051908;

    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as usize as *const u8).read() }
    }
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as usize as *const u32).read_unaligned() }
    }
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as usize as *mut u8).write(value) }
    }
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as usize as *mut u32).write_unaligned(value) }
    }
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }
    unsafe fn lookup_resource(task: u32) -> u32 {
        lf_checker_rt::callee_cdecl!(
            LOOKUP_CALLEE,
            u32,
            unsafe { read_u32(task + TASK_ARGUMENT) },
            0u32,
            lf_checker_rt::relocated(LOOKUP_TABLE_B),
            lf_checker_rt::relocated(LOOKUP_TABLE_A),
            0u32,
        )
    }
    unsafe fn transition_requested(task: u32) -> bool {
        lf_checker_rt::callee_thiscall!(TRANSITION_CALLEE, u8, task, 1u32) != 0
    }
    unsafe fn owner_route_state(owner: u32) -> u32 {
        lf_checker_rt::callee_thiscall!(
            OWNER_ROUTE_CALLEE,
            u32,
            owner.wrapping_add(OWNER_MANAGER),
        )
    }
    unsafe fn owner_state(owner: u32) -> u32 {
        lf_checker_rt::callee_thiscall!(OWNER_STATE_CALLEE, u32, owner)
    }
    unsafe fn resource_is_ready(resource: u32) -> bool {
        lf_checker_rt::callee_thiscall!(RESOURCE_STATE_CALLEE, u8, resource) != 0
    }
    unsafe fn transform_position(resource: u32, output: *mut f32) {
        let _: u32 = lf_checker_rt::callee_cdecl!(
            TRANSFORM_VECTOR_CALLEE,
            u32,
            output as u32,
            resource,
        );
    }
    unsafe fn find_tracker(owner: u32) -> u32 {
        lf_checker_rt::callee_thiscall!(FIND_TRACKER_CALLEE, u32, owner)
    }
    unsafe fn compare_tracker(tracker: u32, vector: *const f32) -> bool {
        lf_checker_rt::callee_thiscall!(COMPARE_TRACKER_CALLEE, u8, tracker, vector as u32) != 0
    }
    unsafe fn inner_radius_resource(pointer: u32) -> u32 {
        lf_checker_rt::callee_thiscall!(INNER_RADIUS_CALLEE, u32, pointer)
    }
    unsafe fn clear_reinitialize(task: u32) {
        let flags = unsafe { read_u8(task + TASK_FLAGS) } & !FORCE_REINITIALIZE;
        unsafe { write_u8(task + TASK_FLAGS, flags) };
    }
    #[inline(always)]
    fn sub_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn mul_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    const USE_TARGET_FLAGS: u8 = 0x08;
    unsafe fn tracking_status(
        task: u32,
        owner: u32,
        resource: u32,
        mut has_transition: bool,
        task_flags: u8,
        task_target: u32,
        task_flags_high: u32,
        resource_flags: u32,
        resource_extra_flags: u32,
        resource_forced_status: u32,
        resource_data: u32,
        resource_tracker: u32,
        track_next: u32,
        track_kind: u32,
        track_rank: u32,
        special_track_kind: u32,
        owner_position: u32,
        position_x: u32,
        use_target_flags: u8,
        use_inner_radius: u8,
        use_outer_radius: u8,
        inner_radius: u32,
        outer_radius: u32,
        inner_radius_callee: u32,
        find_tracker_callee: u32,
        compare_tracker_callee: u32,
        transform_vector_callee: u32,
    ) -> u32 {
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as usize as *const u8).read() }
        }
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as usize as *const u32).read_unaligned() }
        }
        unsafe fn read_f32(address: u32) -> f32 {
            f32::from_bits(unsafe { read_u32(address) })
        }
        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        unsafe {
            let state = read_u32(task + task_target);
            if state == 0 {
                return if has_transition { 0 } else { 3 };
            }
            let flags = read_u8(task + 0xF0);
            if flags & 0x40 == 0 {
                return if has_transition { 0 } else { 3 };
            }
            if read_u32(resource + resource_flags) & 0x3C0 == 0xC0
                && read_u8(resource + resource_extra_flags) & 4 != 0
                && read_u32(resource + resource_forced_status) != 0
            {
                return 3;
            }
            if read_u32(resource + resource_flags) & 0x3C0 == 0xC0 {
                let data = read_u32(resource + resource_data);
                if data != 0 {
                    let mut node = read_u32(data + resource_tracker);
                    if node != 0 {
                        let mut previous_rank = (read_u32(node + track_rank) >> 1) & 7;
                        loop {
                            let current_rank = (read_u32(node + track_rank) >> 1) & 7;
                            if previous_rank < current_rank {
                                break;
                            }
                            if read_u32(node + track_kind) == special_track_kind {
                                return 3;
                            }
                            let next = read_u32(node + track_next);
                            if next == 0 {
                                break;
                            }
                            previous_rank = current_rank;
                            node = next;
                        }
                    }
                }
            }
            if task_flags & use_target_flags == 0 {
                return 0;
            }
            let owner_location = read_u32(owner + owner_position);
            let origin = [
                read_f32(owner_location + position_x),
                read_f32(owner_location + position_x + 4),
                read_f32(owner_location + position_x + 8),
            ];
            let mut transformed = origin;
            let output = transformed.as_mut_ptr();
            let _: u32 = lf_checker_rt::callee_cdecl!(
                transform_vector_callee,
                u32,
                output as u32,
                resource,
            );
            let delta = [
                sub_f32(transformed[0], origin[0]),
                sub_f32(transformed[1], origin[1]),
                sub_f32(transformed[2], origin[2]),
            ];
            let tracker = lf_checker_rt::callee_thiscall!(find_tracker_callee, u32, owner);
            let target = read_u32(task + task_target);
            let mut tracker_accepts = true;
            if tracker != 0 && target != 0 {
                let first = lf_checker_rt::callee_thiscall!(
                    compare_tracker_callee,
                    u8,
                    tracker,
                    delta.as_ptr() as u32,
                ) != 0;
                let second = lf_checker_rt::callee_thiscall!(
                    compare_tracker_callee,
                    u8,
                    tracker,
                    transformed.as_ptr() as u32,
                ) != 0;
                if first && !second {
                    has_transition = false;
                }
                tracker_accepts = first || second;
            }
            let first = mul_f32(delta[0], delta[0]);
            let second = mul_f32(delta[1], delta[1]);
            let sum = add_f32(second, first);
            let distance = add_f32(sum, mul_f32(delta[2], delta[2]));
            if task_flags & use_inner_radius != 0 && tracker_accepts {
                let data = read_u32(resource + resource_data);
                if data != 0 {
                    let inner = lf_checker_rt::callee_thiscall!(inner_radius_callee, u32, data);
                    if inner != 0 {
                        let radius = lf_checker_rt::global::<f32>(inner_radius).read_unaligned();
                        let radius_squared = mul_f32(radius, radius);
                        if radius_squared > distance {
                            return 2;
                        }
                    }
                }
            }
            if !has_transition {
                return 0;
            }
            let radius = lf_checker_rt::global::<f32>(outer_radius).read_unaligned();
            let radius_squared = mul_f32(radius, radius);
            if distance > radius_squared || read_u8(task + task_flags_high) & use_outer_radius != 0 {
                1
            } else {
                0
            }
        }
    }

    unsafe {
        let resource = lookup_resource(task);
        let flags = read_u8(task + TASK_FLAGS);
        if flags & FORCE_REINITIALIZE != 0 {
            clear_reinitialize(task);
            return 3;
        }

        let mut has_transition = transition_requested(task);
        let route_a = read_u8(owner + OWNER_ROUTE_A) != 0;
        let route_b = read_u8(owner + OWNER_ROUTE_B) != 0;
        if !route_a && route_b {
            let route_state = owner_route_state(owner);
            if !has_transition && route_state == 0 {
                clear_reinitialize(task);
                return 3;
            }
        } else {
            return tracking_status(
                task,
                owner,
                resource,
                has_transition,
                flags,
                TASK_TARGET,
                TASK_FLAGS_HIGH,
                RESOURCE_FLAGS,
                RESOURCE_EXTRA_FLAGS,
                RESOURCE_FORCED_STATUS,
                RESOURCE_DATA,
                RESOURCE_TRACKER,
                TRACK_NEXT,
                TRACK_KIND,
                TRACK_RANK,
                TRACK_KIND_SPECIAL,
                OWNER_POSITION,
                POSITION_X,
                USE_TARGET_FLAGS,
                USE_INNER_RADIUS,
                USE_OUTER_RADIUS,
                INNER_RADIUS,
                OUTER_RADIUS,
                INNER_RADIUS_CALLEE,
                FIND_TRACKER_CALLEE,
                COMPARE_TRACKER_CALLEE,
                TRANSFORM_VECTOR_CALLEE,
            );
        }

        let target = read_u32(task + TASK_TARGET);
        let reset_timer = if target != 0 && flags & HAS_TARGET != 0 {
            true
        } else {
            let state = owner_state(owner);
            let changed = read_u8(state + 0x2A4E) ^ read_u8(state + 0x2A4C);
            changed > 0x7F
        };
        let resource_ready = resource_is_ready(resource);
        let duration_address = if resource_ready {
            TRANSITION_DURATION
        } else {
            IDLE_DURATION
        };
        let duration = lf_checker_rt::global::<f32>(duration_address).read_unaligned();
        if reset_timer {
            write_u32(task + TASK_TIMER, 0);
        } else {
            let scale = lf_checker_rt::global::<f32>(TASK_TIME_SCALE).read_unaligned();
            let old_timer = read_f32(task + TASK_TIMER);
            let increment = mul_f32(scale, duration);
            write_u32(task + TASK_TIMER, add_f32(increment, old_timer).to_bits());
        }
        if has_transition && !resource_is_ready(resource) {
            return 0;
        }
        let timer = read_f32(task + TASK_TIMER);
        let threshold = lf_checker_rt::global::<f32>(TIMER_THRESHOLD).read_unaligned();
        if timer > threshold { 3 } else { 0 }
    }
});
