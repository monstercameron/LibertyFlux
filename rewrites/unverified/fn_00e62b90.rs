// original: 0x00e62b90 reset_audio_smoothing_state_g

/// Reset the second global audio smoothing state, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62b90() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x01168ad8;
    const CLEANUP_CALLBACK: u32 = 0x00e711c0;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x01168ad8));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
