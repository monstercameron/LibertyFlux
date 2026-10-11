// original: 0x00e62720 register_graphics_resource_release_b

/// Register the second graphics resource release callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62720() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e71030;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e71030))
});
