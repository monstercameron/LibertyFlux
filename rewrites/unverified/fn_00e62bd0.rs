// original: 0x00e62bd0 register_audio_low_end_footprint_setting

/// Register the audio low-end footprint setting with its global command object and four original stack arguments, then return the helper result.
lf_checker_rt::export!(cdecl, rw_00e62bd0() -> u32 {
    const COMMAND_OBJECT: u32 = 0x01162594;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(COMMAND_OBJECT), 0, lf_checker_rt::relocated(0x00e7e3e4), 0, lf_checker_rt::relocated(0x00e7e3b8))
});
