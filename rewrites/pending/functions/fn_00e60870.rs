// original: 0x00e60870 timing_register_callback_02
/// Register one callback with the registrar helper.
///
/// Passes the code pointer `0x00E6FAB0` to the registrar (stubbed, cdecl/1)
/// and returns its answer.
export!(cdecl, rw_00e60870() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0xE6FAB0)) }
});
