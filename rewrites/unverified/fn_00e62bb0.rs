// original: 0x00e62bb0 register_audio_high_end_footprint_setting

/// Register the audio high-end footprint setting with its global command object and four original stack arguments, then return the helper result.
lf_checker_rt::export!(cdecl, rw_00e62bb0() -> u32 {
    const COMMAND_OBJECT: u32 = 0x011625ac;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(COMMAND_OBJECT), 0, lf_checker_rt::relocated(0x00e7e420), 0, lf_checker_rt::relocated(0x00e7e3f4))
});
