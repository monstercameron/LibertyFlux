// original: 0x00D21140 classify_time_slot
/// Classifies a value after adding the shared clock word. A nonzero low byte of `flag` selects the 40,000-unit period; zero selects 25,000. The return codes are 2 below the first boundary, 0 in the middle interval, and 1 at or above the upper boundary.
lf_checker_rt::export!(cdecl, rw_00d21140(flag: u32, value: u32) -> u32 {
    const GLOBAL_VALUE_FILE_VA: u32 = 0x0117_3608;
    const LONG_PERIOD: u32 = 40_000;
    const SHORT_PERIOD: u32 = 25_000;
    const LONG_LOW: u32 = 27_500;
    const LONG_HIGH: u32 = 37_500;
    const SHORT_LOW: u32 = 12_500;
    const SHORT_HIGH: u32 = 22_500;

    let clock_value = unsafe { lf_checker_rt::global::<u32>(GLOBAL_VALUE_FILE_VA).read_unaligned() }.wrapping_add(value);
    if flag & 0xff != 0 {
        let remainder = clock_value % LONG_PERIOD;
        if remainder < LONG_LOW { 2 } else if remainder < LONG_HIGH { 0 } else { 1 }
    } else {
        let remainder = clock_value % SHORT_PERIOD;
        if remainder < SHORT_LOW { 2 } else if remainder < SHORT_HIGH { 0 } else { 1 }
    }
});
