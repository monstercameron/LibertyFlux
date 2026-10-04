// original: 0x00e5e5b0 net_init_register_5b0
/// Run the unit initializer, install the dispatch target, register the handler.
///
/// The original calls the unit initializer (no arguments), stores the
/// dispatch routine address into its dispatch slot, then pushes the handler
/// address, calls the shared registrar and returns the registrar's answer.
/// The callees are intercepted by the checker; the slot is a plain global.
export!(cdecl, rw_00e5e5b0() -> u32 {
    unsafe {
        /// Handler routine this unit registers (file VA).
        const HANDLER: u32 = 0x00E6F060;
        /// Dispatch slot this unit installs its target into (file VA).
        const DISPATCH_SLOT: u32 = 0x019E8678;
        /// Dispatch routine address installed (file VA).
        const DISPATCH_TARGET: u32 = 0x00FE0BD8;
        let _: u32 = callee_cdecl!(1, u32,);
        global::<u32>(DISPATCH_SLOT).write(relocated(DISPATCH_TARGET));
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
