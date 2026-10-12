// original: 0x00D38710 CTaskComplexClearVehicleSeat::get_status_name

/// Return the original image pointer for the status label selected by the
/// unsigned status id. Each defined id has a distinct relocated string pointer. All larger 32-bit values return null. The
/// function is cdecl with one stack argument and has no memory writes.
lf_checker_rt::export!(cdecl, rw_00D38710(status_id: u32) -> u32 {
    const LAST_STATUS: u32 = 5;
    const STATUS_STRING_0: u32 = 0x00EE242C;
    const STATUS_STRING_1: u32 = 0x00EE243C;
    const STATUS_STRING_2: u32 = 0x00EE2450;
    const STATUS_STRING_3: u32 = 0x00EE2464;
    const STATUS_STRING_4: u32 = 0x00EE2474;
    const STATUS_STRING_5: u32 = 0x00EE2490;
    if status_id > LAST_STATUS {
        return 0;
    }
    match status_id {
        0 => lf_checker_rt::relocated(STATUS_STRING_0),
        1 => lf_checker_rt::relocated(STATUS_STRING_1),
        2 => lf_checker_rt::relocated(STATUS_STRING_2),
        3 => lf_checker_rt::relocated(STATUS_STRING_3),
        4 => lf_checker_rt::relocated(STATUS_STRING_4),
        5 => lf_checker_rt::relocated(STATUS_STRING_5),
        _ => 0
    }
});
