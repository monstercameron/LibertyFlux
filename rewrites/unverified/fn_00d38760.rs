// original: 0x00D38760 CTaskComplexClimbIntoVehicle::get_status_name

/// Return the original image pointer for the status label selected by the
/// unsigned status id. Every id from 0 through 10 has a distinct
/// relocated string pointer; all larger 32-bit values return null. The
/// function is cdecl with one stack argument and has no memory writes.
lf_checker_rt::export!(cdecl, rw_00D38760(status_id: u32) -> u32 {
    const LAST_STATUS: u32 = 10;
    const STATUS_STRING_0: u32 = 0x00EE2780;
    const STATUS_STRING_1: u32 = 0x00EE266C;
    const STATUS_STRING_2: u32 = 0x00EE2790;
    const STATUS_STRING_3: u32 = 0x00EE27A8;
    const STATUS_STRING_4: u32 = 0x00EE27BC;
    const STATUS_STRING_5: u32 = 0x00EE27D0;
    const STATUS_STRING_6: u32 = 0x00EE27EC;
    const STATUS_STRING_7: u32 = 0x00EE2804;
    const STATUS_STRING_8: u32 = 0x00EE281C;
    const STATUS_STRING_9: u32 = 0x00EE2834;
    const STATUS_STRING_10: u32 = 0x00EE2850;
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
        6 => lf_checker_rt::relocated(STATUS_STRING_6),
        7 => lf_checker_rt::relocated(STATUS_STRING_7),
        8 => lf_checker_rt::relocated(STATUS_STRING_8),
        9 => lf_checker_rt::relocated(STATUS_STRING_9),
        10 => lf_checker_rt::relocated(STATUS_STRING_10),
        _ => 0
    }
});
