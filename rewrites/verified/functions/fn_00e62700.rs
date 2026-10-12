// original: 0x00e62700 register_graphics_state_object_cleanup_c

/// Register the third graphics state object's cleanup callback and return the registration result. The cdecl entry has no arguments; it forwards the cleanup pointer to the cdecl registrar and returns the registrar's EAX result.
lf_checker_rt::export!(cdecl, rw_00e62700() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70ff0;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
