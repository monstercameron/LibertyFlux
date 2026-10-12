// original: 0x00E6AB30 initialize_vehicle_audio_link_head
/// Reproduce initialize_vehicle_audio_link_head at 0x00e6ab30. This zero-argument cdecl routine has no semantic return.
///
/// Call the direct initializer with global object 0x016cfd4c in ECX, then register callback 0x00e72a60.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6ab30() -> () {
    const OBJECT: u32 = 0x016cfd4c;
    const CALLBACK: u32 = 0x00e72a60;
    let _ = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJECT));
    let _ = lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(CALLBACK));
    
});
