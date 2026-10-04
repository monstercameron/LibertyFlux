// original: 0x00e5e4b0 net_register_e4b0
/// Register one network handler, returning the registrar's answer.
///
/// Forwards the handler at `0x00E6EF00` to the registrar. No state of its own.
export!(cdecl, rw_00e5e4b0() -> u32 {
    unsafe {
        /// Handler registered (file VA).
        const HANDLER: u32 = 0x00E6EF00;
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
