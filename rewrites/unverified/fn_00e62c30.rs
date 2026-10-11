// original: 0x00e62c30 register_audio_buffer_cleanup

/// Register the audio buffer cleanup callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62c30() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e711e0;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e711e0))
});
