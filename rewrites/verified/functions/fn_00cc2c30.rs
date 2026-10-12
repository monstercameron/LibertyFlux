// original: 0x00CC2C30 process_route_tasks
/// Walk the current task list. When the global feature weight is at least two,
/// set high-priority task types to -4.0 or update the first unblocked route
/// match to -1.0 when its distance exceeds -1.0. Remove a zero-speed task when
/// its distance is non-positive. Task comparisons use exact integer equality;
/// distance comparisons are ordered f32 comparisons.
lf_checker_rt::export!(thiscall, rw_00cc2c30(this: *mut u8, route_entries: *const u8) -> u32 {
    const MANAGER_OFFSET: usize = 0x24;
    const TASK_MANAGER_OFFSET: usize = 0x78;
    const TASK_TYPE_OFFSET: usize = 0x0C;
    const TASK_SPEED_OFFSET: usize = 0x58;
    const ROUTE_TYPES_OFFSET: usize = 0xB0;
    const ROUTE_FLAGS_OFFSET: usize = 0xB4;
    const ROUTE_STRIDE: usize = 0x18;
    const ROUTE_ENTRY_COUNT: usize = 7;
    const FEATURE_WEIGHT: u32 = 0x0171C0DC;
    const MIN_FEATURE_WEIGHT: f32 = 2.0;
    const MIN_ROUTE_DISTANCE: f32 = -1.0;
    const ZERO_SPEED: f32 = 0.0;
    const NEGATIVE_ONE_BITS: u32 = 0xBF800000;
    const NEGATIVE_FOUR_BITS: u32 = 0xC0800000;
    const FIRST_TASK_INDEX: u32 = 0;
    const TASK_QUERY_LIMIT: u32 = 2;
    const HIGH_PRIORITY_TYPES: [u32; 5] = [0x26,0x27,0x30,0x31,0x35];
    let manager = unsafe { this.add(MANAGER_OFFSET).cast::<u32>().read_unaligned() } as *const u8;
    let task_manager = unsafe { manager.add(TASK_MANAGER_OFFSET).cast::<u32>().read_unaligned() };
    let feature_weight = unsafe { f32::from_bits(lf_checker_rt::global::<u32>(FEATURE_WEIGHT).read_unaligned()) };
    let mut task = lf_checker_rt::callee_thiscall!(1, u32, task_manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    while task != 0 {
        let task_ptr = task as *const u8;
        let task_type = unsafe { task_ptr.add(TASK_TYPE_OFFSET).cast::<u32>().read_unaligned() };
        if !(feature_weight < MIN_FEATURE_WEIGHT || feature_weight.is_nan()) {
            if HIGH_PRIORITY_TYPES.contains(&task_type) {
                let _ = lf_checker_rt::callee_thiscall!(3, u32, task, NEGATIVE_FOUR_BITS);
            } else {
                for entry in 0..ROUTE_ENTRY_COUNT {
                    let entry_offset = entry * ROUTE_STRIDE;
                    let flags = unsafe { route_entries.add(ROUTE_FLAGS_OFFSET + entry_offset).read() };
                    if flags & 1 != 0 {
                        continue;
                    }
                    let route_type = unsafe { route_entries.add(ROUTE_TYPES_OFFSET + entry_offset).cast::<u32>().read_unaligned() };
                    if route_type == task_type {
                        let distance = lf_checker_rt::callee_thiscall!(2, f32, task);
                        if distance > MIN_ROUTE_DISTANCE {
                            let _ = lf_checker_rt::callee_thiscall!(3, u32, task, NEGATIVE_ONE_BITS);
                            break;
                        }
                    }
                }
            }
        }
        let speed = unsafe { f32::from_bits(task_ptr.add(TASK_SPEED_OFFSET).cast::<u32>().read_unaligned()) };
        if speed == ZERO_SPEED {
            let distance = lf_checker_rt::callee_thiscall!(2, f32, task);
            if distance <= 0.0 {
                let _ = lf_checker_rt::callee_thiscall!(4, u32, task_manager, task);
            }
        }
        task = lf_checker_rt::callee_thiscall!(5, u32, task_manager, FIRST_TASK_INDEX, TASK_QUERY_LIMIT);
    }
    0
});
