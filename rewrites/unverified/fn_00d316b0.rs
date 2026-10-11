// original: 0x00D316B0 get_vehicle_seat_sub_task
/// Returns whether the requested vehicle subtask exists. It first rejects a
/// disabled task or a vehicle in terminal state, then fetches two seat records
/// and compares their classifier results as signed 32-bit values. When the
/// second result is greater, it resolves the vehicle slot and forwards the
/// original candidate, slot pointer and task flags to the final helper.
lf_checker_rt::export!(thiscall, rw_00d316b0(task: u32, candidate: u32, record_array: u32) -> u8 {
    const TASK_FLAGS: u32 = 0x14;
    const VEHICLE_POINTER: u32 = 0x18;
    const SEAT_INDEX: u32 = 0x24;
    const VEHICLE_STATE: u32 = 0x1304;
    const DISABLED_FLAG: u32 = 0x0400_0000;
    const TERMINAL_STATE: u32 = 3;

    #[inline(never)]
    fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }

    let task_flags = read_u32(task.wrapping_add(TASK_FLAGS));
    if task_flags & DISABLED_FLAG != 0 { return 0; }
    let vehicle = read_u32(task.wrapping_add(VEHICLE_POINTER));
    if read_u32(vehicle.wrapping_add(VEHICLE_STATE)) == TERMINAL_STATE { return 0; }

    const ARRAY_GET_INDEX_ONE: u32 = 1;
    const ARRAY_GET_INDEX_ZERO: u32 = 0;
    let selected = lf_checker_rt::callee_thiscall!(1, u32, record_array, ARRAY_GET_INDEX_ONE);
    if selected == u32::MAX { return 0; }

    let seat = read_u32(task.wrapping_add(SEAT_INDEX));
    let first_record = lf_checker_rt::callee_thiscall!(1, u32, record_array, ARRAY_GET_INDEX_ZERO);
    let first_category = lf_checker_rt::callee_cdecl!(2, u32, seat, first_record);
    let second_record = lf_checker_rt::callee_thiscall!(1, u32, record_array, ARRAY_GET_INDEX_ONE);
    let second_category = lf_checker_rt::callee_cdecl!(2, u32, seat, second_record);
    if (second_category as i32) <= (first_category as i32) { return 0; }

    let slot_index = lf_checker_rt::callee_thiscall!(1, u32, record_array, ARRAY_GET_INDEX_ONE);
    let vehicle_slot = lf_checker_rt::callee_thiscall!(3, u32, vehicle, slot_index);
    lf_checker_rt::callee_cdecl!(4, u8, candidate, vehicle_slot, task_flags)
});
