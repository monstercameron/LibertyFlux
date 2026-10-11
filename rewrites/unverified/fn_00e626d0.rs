// original: 0x00e626d0 initialize_graphics_state_object_b

/// Initialize the second global graphics state object, register its cleanup callback, and return the registration result.
lf_checker_rt::export!(cdecl, rw_00e626d0() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x0115a4d4;
    const CLEANUP_CALLBACK: u32 = 0x00e70fd0;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0115a4d4));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
