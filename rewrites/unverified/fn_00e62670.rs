// original: 0x00e62670 register_global_graphics_object_cleanup

/// Register the global graphics object cleanup callback and return the registration result. The cdecl entry has no arguments; it forwards the cleanup pointer to the cdecl registrar and returns the registrar's EAX result.
lf_checker_rt::export!(cdecl, rw_00e62670() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70fa0;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
