// original: 0x00e62670 register_global_graphics_object_cleanup

/// Register the global graphics object cleanup callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e62670() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70fa0;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e70fa0))
});
