// original: 0x00e62cc0 register_network_global_pointer_initializer_c

/// Register the third network global-pointer initializer cleanup callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62cc0() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e71230;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e71230))
});
