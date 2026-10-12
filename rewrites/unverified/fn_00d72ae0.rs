// original: 0x00D72AE0 replay_marker_mode_is_listed_mode

/// Returns one for mode values 12, 13, 14, 15, or 17 and zero for every other 32-bit value.
lf_checker_rt::export!(stdcall, rw_00d72ae0(mode: u32) -> u8 {
    u8::from(matches!(mode, 12 | 14 | 17 | 15 | 13))
});
