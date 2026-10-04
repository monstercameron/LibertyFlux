// original: 0x00e5e340 net_register_e340
/// Register one network handler, returning the registrar's answer.
///
/// Forwards the handler at `0x00E6EEA0` to the registrar. No state of its own.
export!(cdecl, rw_00e5e340() -> u32 {
    unsafe {
        /// Handler registered (file VA).
        const HANDLER: u32 = 0x00E6EEA0;
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
