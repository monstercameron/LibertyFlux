// original: 0x00E6A6D0 register_vehicle_state_cleanup
/// Reproduce register_vehicle_state_cleanup at 0x00e6a6d0. This zero-argument cdecl routine has no semantic return.
///
/// Pass the cleanup callback at 0x00e728d0 to the cdecl cleanup registrar.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a6d0() -> () {
    const CALLBACK: u32 = 0x00e728d0;
    let _ = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK));
    
});
