// original: 0x00D38910 CTaskComplexNewGetInVehicle::get_status_name

/// Return the original image pointer for the status label selected by the
/// unsigned status id. Defined ids map to relocated string pointers; some ids share a pointer. All larger 32-bit values return null. The
/// function is cdecl with one stack argument and has no memory writes.
lf_checker_rt::export!(cdecl, rw_00D38910(status_id: u32) -> u32 {
    const LAST_STATUS: u32 = 34;
    const STATUS_STRING_0: u32 = 0x00EE2CC0;
    const STATUS_STRING_1: u32 = 0x00EE2CD0;
    const STATUS_STRING_2: u32 = 0x00EE2CE4;
    const STATUS_STRING_3: u32 = 0x00EE2CFC;
    const STATUS_STRING_4: u32 = 0x00EE2D74;
    const STATUS_STRING_5: u32 = 0x00EE2DA0;
    const STATUS_STRING_6: u32 = 0x00EE2D18;
    const STATUS_STRING_7: u32 = 0x00EE2D34;
    const STATUS_STRING_8: u32 = 0x00EE2D54;
    const STATUS_STRING_9: u32 = 0x00EE2D88;
    const STATUS_STRING_10: u32 = 0x00EE2DA0;
    const STATUS_STRING_11: u32 = 0x00EE2DB4;
    const STATUS_STRING_12: u32 = 0x00EE2DC8;
    const STATUS_STRING_13: u32 = 0x00EE2DD8;
    const STATUS_STRING_14: u32 = 0x00EE2DF0;
    const STATUS_STRING_15: u32 = 0x00EE2E04;
    const STATUS_STRING_16: u32 = 0x00EE2E1C;
    const STATUS_STRING_17: u32 = 0x00EE2E7C;
    const STATUS_STRING_18: u32 = 0x00EE2E94;
    const STATUS_STRING_19: u32 = 0x00EE2EA8;
    const STATUS_STRING_20: u32 = 0x00EE2EBC;
    const STATUS_STRING_21: u32 = 0x00EE2ED8;
    const STATUS_STRING_22: u32 = 0x00EE2E50;
    const STATUS_STRING_23: u32 = 0x00EE2E2C;
    const STATUS_STRING_24: u32 = 0x00EE2EEC;
    const STATUS_STRING_25: u32 = 0x00EE2F00;
    const STATUS_STRING_26: u32 = 0x00EE2F18;
    const STATUS_STRING_27: u32 = 0x00EE2F44;
    const STATUS_STRING_28: u32 = 0x00EE22D8;
    const STATUS_STRING_29: u32 = 0x00EE22F0;
    const STATUS_STRING_30: u32 = 0x00EE2300;
    const STATUS_STRING_31: u32 = 0x00EE2310;
    const STATUS_STRING_32: u32 = 0x00EE2334;
    const STATUS_STRING_33: u32 = 0x00EE2364;
    const STATUS_STRING_34: u32 = 0x00EE2378;
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
        26 => lf_checker_rt::relocated(STATUS_STRING_26),
        27 => lf_checker_rt::relocated(STATUS_STRING_27),
        28 => lf_checker_rt::relocated(STATUS_STRING_28),
        29 => lf_checker_rt::relocated(STATUS_STRING_29),
        30 => lf_checker_rt::relocated(STATUS_STRING_30),
        31 => lf_checker_rt::relocated(STATUS_STRING_31),
        32 => lf_checker_rt::relocated(STATUS_STRING_32),
        33 => lf_checker_rt::relocated(STATUS_STRING_33),
        34 => lf_checker_rt::relocated(STATUS_STRING_34),
        _ => 0
    }
});
