// original: 0x00E6A720 initialize_vehicle_object_buffer_slots
/// Reproduce initialize_vehicle_object_buffer_slots at 0x00e6a720. This zero-argument cdecl routine has no semantic return.
///
/// Initialize 32 records at stride 0x3d0 through two array helpers and two per-record thiscall helpers, apply the flag mask, then register cleanup.
///
/// File VAs are resolved through the checker runtime. Direct callees are intercepted; their bodies are outside this function's contract.
lf_checker_rt::export!(cdecl, rw_00e6a720() -> () {
    unsafe {
        const BASE: u32 = 0x016ba304;
        const COUNT: usize = 32;
        const STRIDE: u32 = 0x3d0;
        const INITIALIZER: u32 = 0x004065e0;
        const ARRAY_A_OFFSET: u32 = 0x2b4;
        const ARRAY_A_WIDTH: u32 = 0x10;
        const ARRAY_A_COUNT: u32 = 0x11;
        const ARRAY_B_OFFSET: u32 = 0x114;
        const ARRAY_B_WIDTH: u32 = 0x20;
        const ARRAY_B_COUNT: u32 = 8;
        const INITIALIZED_OBJECT_OFFSET: u32 = 0x0c;
        const FLAGS_OFFSET: u32 = 4;
        const HOLDER_OFFSET: u32 = 0x4c;
        const FLAGS_TO_CLEAR: u32 = 0x0f;
        const CLEANUP_CALLBACK: u32 = 0x00e728f0;
        for index in 0..COUNT {
            let row = BASE.wrapping_add((index as u32).wrapping_mul(STRIDE));
            let _ = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(row.wrapping_sub(ARRAY_A_OFFSET)), ARRAY_A_WIDTH, ARRAY_A_COUNT, lf_checker_rt::relocated(INITIALIZER));
            let _ = lf_checker_rt::callee_stdcall!(1, u32, lf_checker_rt::relocated(row.wrapping_sub(ARRAY_B_OFFSET)), ARRAY_B_WIDTH, ARRAY_B_COUNT, lf_checker_rt::relocated(INITIALIZER));
            let _ = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(row.wrapping_add(INITIALIZED_OBJECT_OFFSET)));
            let flags = lf_checker_rt::global::<u32>(row.wrapping_sub(FLAGS_OFFSET));
            flags.write(flags.read() & !FLAGS_TO_CLEAR);
            lf_checker_rt::global::<u16>(row).write(u16::MAX);
            let _ = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(row.wrapping_add(HOLDER_OFFSET)));
        }
        let _ = lf_checker_rt::callee_cdecl!(4, u32, lf_checker_rt::relocated(CLEANUP_CALLBACK));
    }
});
