// original: 0x00E6A640 clear_vehicle_static_state
/// Reproduce clear_vehicle_static_state at 0x00e6a640. This zero-argument cdecl routine has no semantic return.
///
/// Clear eight global dwords to zero, then register the cleanup callback.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a640() -> () {
    unsafe {
        const STATE_WORDS: [u32; 8] = [0x016c4118, 0x016c4368, 0x016c45b8, 0x016c4808, 0x016c4a58, 0x016c4ca8, 0x016c4ef8, 0x016c5148];
        const CALLBACK: u32 = 0x00e72870;
        for address in STATE_WORDS {
            lf_checker_rt::global::<u32>(address).write(0);
        }
        let _ = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK));
    }
});
