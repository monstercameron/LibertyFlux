// original: 0x00E6A8B0 rain_particle_settings_initialize_and_register_cleanup
/// Reproduce rain_particle_settings_initialize_and_register_cleanup at 0x00e6a8b0. This zero-argument cdecl routine has no semantic return.
///
/// Store relocated read-only pointer 0x00ec5e14 in two global dwords and register cleanup.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a8b0() -> () {
    unsafe {
        const SETTINGS: u32 = 0x00ec5e14;
        const SETTINGS_FIELDS: [u32; 2] = [0x016c86e0, 0x016c87b0];
        const CALLBACK: u32 = 0x00e729b0;
        for field in SETTINGS_FIELDS {
            lf_checker_rt::global::<u32>(field).write(lf_checker_rt::relocated(SETTINGS));
        }
        let _ = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(CALLBACK));
    }
});
