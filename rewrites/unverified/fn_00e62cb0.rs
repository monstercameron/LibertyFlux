// original: 0x00e62cb0 register_network_global_pointer_initializer_b

/// Register the second network global-pointer initializer cleanup callback and return the registration result. The cdecl entry has no arguments; it forwards the cleanup pointer to the cdecl registrar and returns the registrar's EAX result.
lf_checker_rt::export!(cdecl, rw_00e62cb0() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e71220;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
