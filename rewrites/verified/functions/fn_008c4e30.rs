// original: 0x008C4E30 NativeImpl_SAVE_SETTINGS_2

/// Forward one stack argument and the fixed settings identifier to the intercepted cdecl helper, then return its result.
lf_checker_rt::export!(cdecl, rw_008c4e30(argument: u32) -> u32 {
    const CALLEE_ID: u32 = 30;
    let _ = CALLEE_ID;
    unsafe { lf_checker_rt::callee_cdecl!(30, u32, argument, lf_checker_rt::relocated(0x00E7_F734)) }
});
