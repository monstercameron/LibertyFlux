// original: 0x00e5e560 net_handler_register_560
/// Register this unit's handler routine with the shared registrar.
///
/// The original pushes the handler address and calls the registrar, which
/// answers 0 or 1 (it boolean-transforms an inner lookup); the stub returns
/// that answer unchanged. The registrar is intercepted by the checker, so
/// this function is fully described by the one outgoing call and its answer.
export!(cdecl, rw_00e5e560() -> u32 {
    unsafe {
        /// Handler routine this unit registers (file VA).
        const HANDLER: u32 = 0x00E6F020;
        callee_cdecl!(1, u32, relocated(HANDLER))
    }
});
