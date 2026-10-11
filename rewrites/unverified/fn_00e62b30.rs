// original: 0x00e62b30 initialize_audio_link_head_and_register_cleanup_c

/// Initialize the first global audio link head, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62b30() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x01168b64;
    const CLEANUP_CALLBACK: u32 = 0x00e71190;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x01168b64));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
