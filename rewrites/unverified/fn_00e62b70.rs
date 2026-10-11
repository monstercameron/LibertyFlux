// original: 0x00e62b70 reset_audio_smoothing_state_f

/// Reset the first global audio smoothing state, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62b70() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x01162614;
    const CLEANUP_CALLBACK: u32 = 0x00e711b0;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x01162614));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
