// original: 0x00e62630 initialize_network_state_object_b

/// Initialize the second global network state object, register its cleanup callback, and return the registration result. The cdecl entry has no arguments; it calls the initializer as thiscall with the global object in ECX, then calls the cdecl cleanup registrar with the callback pointer.
lf_checker_rt::export!(cdecl, rw_00e62630() -> u32 {
    const GLOBAL_OBJECT: u32 = 0x011540a0;
    const CLEANUP_CALLBACK: u32 = 0x00e70f80;
    let _initializer_result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(GLOBAL_OBJECT));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
