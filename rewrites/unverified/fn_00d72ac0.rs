// original: 0x00D72AC0 replay_marker_mode_is_10_or_11

/// Returns one when the signedness-independent mode value equals 10 or 11, otherwise zero.
lf_checker_rt::export!(stdcall, rw_00d72ac0(mode: u32) -> u8 {
    u8::from(matches!(mode, 10 | 11))
});
