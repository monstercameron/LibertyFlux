// original: 0x00D38A80 CTaskComplexOpenVehicleDoor::get_status_name

/// Return the original image pointer for the status label selected by the
/// unsigned status id. Each defined id has a distinct relocated string pointer. All larger 32-bit values return null. The
/// function is cdecl with one stack argument and has no memory writes.
lf_checker_rt::export!(cdecl, rw_00D38A80(status_id: u32) -> u32 {
    const LAST_STATUS: u32 = 4;
    const STATUS_STRING_0: u32 = 0x00EE2C40;
    const STATUS_STRING_1: u32 = 0x00EE2C50;
    const STATUS_STRING_2: u32 = 0x00EE2C68;
    const STATUS_STRING_3: u32 = 0x00EE2C84;
    const STATUS_STRING_4: u32 = 0x00EE2C9C;
    if status_id > LAST_STATUS {
        return 0;
    }
    match status_id {
        0 => lf_checker_rt::relocated(STATUS_STRING_0),
        1 => lf_checker_rt::relocated(STATUS_STRING_1),
        2 => lf_checker_rt::relocated(STATUS_STRING_2),
        3 => lf_checker_rt::relocated(STATUS_STRING_3),
        4 => lf_checker_rt::relocated(STATUS_STRING_4),
        _ => 0
    }
});
