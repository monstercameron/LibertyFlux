// original: 0x00e5e270 net_slot_bind_e270
/// Initialise one static network object, then register its handler.
///
/// Runs the tiny object initializer on the static object at `0x019D3024`,
/// then registers the handler at `0x00E6EDF0` and returns the registrar's
/// answer. The initializer's answer is discarded.
export!(cdecl, rw_00e5e270() -> u32 {
    unsafe {
        /// Static object this instance initialises (file VA).
        const OBJ: u32 = 0x019D3024;
        /// Handler registered for it (file VA).
        const HANDLER: u32 = 0x00E6EDF0;
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ));
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
