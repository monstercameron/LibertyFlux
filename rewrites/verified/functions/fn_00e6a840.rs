// original: 0x00E6A840 nav_request_store_initialize_and_register_cleanup
/// Reproduce nav_request_store_initialize_and_register_cleanup at 0x00e6a840. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer with global object 0x016c5360 in ECX, then register callback 0x00e72980.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a840() -> () {
    const OBJECT: u32 = 0x016c5360;
    const CALLBACK: u32 = 0x00e72980;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT));
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK));
    
});
