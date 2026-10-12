// original: 0x00e62160 register_network_thread_pool_cleanup

/// Register the network thread-pool cleanup callback and return the registration result. The cdecl entry has no arguments; it forwards the cleanup pointer to the cdecl registrar and returns the registrar's EAX result.
lf_checker_rt::export!(cdecl, rw_00e62160() -> u32 {
    const CLEANUP_CALLBACK: u32 = 0x00e70960;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK))
});
