// original: 0x00D387F0 CTaskComplexNewExitVehicle::get_status_name

/// Return the original image pointer for the status label selected by the
/// unsigned status id. Each defined id has a distinct relocated string pointer. All larger 32-bit values return null. The
/// function is cdecl with one stack argument and has no memory writes.
lf_checker_rt::export!(cdecl, rw_00D387F0(status_id: u32) -> u32 {
    const LAST_STATUS: u32 = 25;
    const STATUS_STRING_0: u32 = 0x00EE2894;
    const STATUS_STRING_1: u32 = 0x00EE28A4;
    const STATUS_STRING_2: u32 = 0x00EE28BC;
    const STATUS_STRING_3: u32 = 0x00EE28D0;
    const STATUS_STRING_4: u32 = 0x00EE28E8;
    const STATUS_STRING_5: u32 = 0x00EE2900;
    const STATUS_STRING_6: u32 = 0x00EE291C;
    const STATUS_STRING_7: u32 = 0x00EE2934;
    const STATUS_STRING_8: u32 = 0x00EE2958;
    const STATUS_STRING_9: u32 = 0x00EE2990;
    const STATUS_STRING_10: u32 = 0x00EE29B4;
    const STATUS_STRING_11: u32 = 0x00EE29C4;
    const STATUS_STRING_12: u32 = 0x00EE29D8;
    const STATUS_STRING_13: u32 = 0x00EE29EC;
    const STATUS_STRING_14: u32 = 0x00EE2A00;
    const STATUS_STRING_15: u32 = 0x00EE2A1C;
    const STATUS_STRING_16: u32 = 0x00EE2A34;
    const STATUS_STRING_17: u32 = 0x00EE2A4C;
    const STATUS_STRING_18: u32 = 0x00EE2A64;
    const STATUS_STRING_19: u32 = 0x00EE2A74;
    const STATUS_STRING_20: u32 = 0x00EE2A84;
    const STATUS_STRING_21: u32 = 0x00EE2A9C;
    const STATUS_STRING_22: u32 = 0x00EE2AB4;
    const STATUS_STRING_23: u32 = 0x00EE2ACC;
    const STATUS_STRING_24: u32 = 0x00EE2ADC;
    const STATUS_STRING_25: u32 = 0x00EE2AF0;
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
        11 => lf_checker_rt::relocated(STATUS_STRING_11),
        12 => lf_checker_rt::relocated(STATUS_STRING_12),
        13 => lf_checker_rt::relocated(STATUS_STRING_13),
        14 => lf_checker_rt::relocated(STATUS_STRING_14),
        15 => lf_checker_rt::relocated(STATUS_STRING_15),
        16 => lf_checker_rt::relocated(STATUS_STRING_16),
        17 => lf_checker_rt::relocated(STATUS_STRING_17),
        18 => lf_checker_rt::relocated(STATUS_STRING_18),
        19 => lf_checker_rt::relocated(STATUS_STRING_19),
        20 => lf_checker_rt::relocated(STATUS_STRING_20),
        21 => lf_checker_rt::relocated(STATUS_STRING_21),
        22 => lf_checker_rt::relocated(STATUS_STRING_22),
        23 => lf_checker_rt::relocated(STATUS_STRING_23),
        24 => lf_checker_rt::relocated(STATUS_STRING_24),
        25 => lf_checker_rt::relocated(STATUS_STRING_25),
        _ => 0
    }
});
