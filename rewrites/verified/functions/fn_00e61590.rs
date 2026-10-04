// original: 0x00e61590 timing_register_callback_c
/// Register one timing callback, returning the registrar's answer.
export!(cdecl, rw_00e61590() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E70200)) }
});
