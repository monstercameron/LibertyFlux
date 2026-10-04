// original: 0x00e61650 timing_pair_zero_and_register
/// Zero two timing counters, then register the callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e61650() -> u32 {
    unsafe {
        *global::<u32>(0x01A02214) = 0;
        *global::<u32>(0x01A02218) = 0;
        callee_cdecl!(1, u32, relocated(0x00E70220))
    }
});
