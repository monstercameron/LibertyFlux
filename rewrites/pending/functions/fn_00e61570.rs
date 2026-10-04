// original: 0x00e61570 timing_register_callback_a
/// Register one timing callback, returning the registrar's answer.
export!(cdecl, rw_00e61570() -> u32 {
    unsafe { callee_cdecl!(1, u32, relocated(0x00E701E0)) }
});
