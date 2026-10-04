// original: 0x00e5e520 net_init_register_520
/// Run the unit initializer, then register this unit's handler routine.
///
/// The original first calls the zero-fill initializer (it clears its static
/// state and needs no arguments), then pushes the handler address, calls the
/// shared registrar and returns the registrar's answer. Both callees are
/// intercepted by the checker, so this function is fully described by the
/// two outgoing calls in order and the returned answer.
export!(cdecl, rw_00e5e520() -> u32 {
    unsafe {
        /// Handler routine this unit registers (file VA).
        const HANDLER: u32 = 0x00E6EFA0;
        let _: u32 = callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
