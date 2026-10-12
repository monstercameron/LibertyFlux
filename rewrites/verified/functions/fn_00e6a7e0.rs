// original: 0x00E6A7E0 initialize_vehicle_static_store_a_and_register_cleanup
/// Reproduce initialize_vehicle_static_store_a_and_register_cleanup at 0x00e6a7e0. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer with global object 0x016c6710 in ECX, then register callback 0x00e72950.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a7e0() -> () {
    const OBJECT: u32 = 0x016c6710;
    const CALLBACK: u32 = 0x00e72950;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT));
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK));
    
});
