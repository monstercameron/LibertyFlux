// original: 0x00B7EA00 get_car_drive_status_label

/// Return the static label for a car-drive status kind from the unsigned
/// one-word input. Kinds zero through six each select one label; all larger
/// values, including values with the top bit set, return null.
lf_checker_rt::export!(cdecl, rw_00b7ea00(kind: u32) -> u32 {
    const MAX_STATUS_KIND: u32 = 6;

    if kind > MAX_STATUS_KIND {
        return 0;
    }

    unsafe {
        match kind {
            0 => lf_checker_rt::global::<u8>(0x00EB31BC) as u32,
            1 => lf_checker_rt::global::<u8>(0x00EB2DE8) as u32,
            2 => lf_checker_rt::global::<u8>(0x00EB2DFC) as u32,
            3 => lf_checker_rt::global::<u8>(0x00EB2E0C) as u32,
            4 => lf_checker_rt::global::<u8>(0x00EB2E1C) as u32,
            5 => lf_checker_rt::global::<u8>(0x00EB2E30) as u32,
            6 => lf_checker_rt::global::<u8>(0x00EB2E3C) as u32,
            _ => 0,
        }
    }
});
