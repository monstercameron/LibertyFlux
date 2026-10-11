// original: 0x00e626c0 register_graphics_state_object_cleanup_a

/// Register the first graphics state object's cleanup callback and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e626c0() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70fc0;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00e70fc0))
});
