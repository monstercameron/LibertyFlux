// original: 0x00e607e0 timing_register_callback_01
/// Register one callback with the registrar helper.
///
/// Passes the code pointer `0x00E6FA60` to the registrar (stubbed, cdecl/1)
/// and returns its answer.
export!(cdecl, rw_00e607e0() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE6FA60)) }
});
