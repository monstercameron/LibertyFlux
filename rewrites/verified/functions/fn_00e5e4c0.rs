// original: 0x00e5e4c0 net_setup_register_e4c0
/// Run the network setup step, then register the handler.
///
/// Calls the setup routine (its answer is discarded), then registers the
/// handler at `0x00E6EF10` and returns the registrar's answer.
export!(cdecl, rw_00e5e4c0() -> u32 {
    unsafe {
        /// Handler registered (file VA).
        const HANDLER: u32 = 0x00E6EF10;
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
