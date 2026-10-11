// original: 0x00e62710 register_graphics_resource_release_a

/// Register the first graphics resource release callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62710() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e71010;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e71010))
});
