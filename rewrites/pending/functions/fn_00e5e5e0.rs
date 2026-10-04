// original: 0x00e5e5e0 net_tlsobj_init_register_5e0
/// Initialise the unit's shared object, then register its handler routine.
///
/// The original calls the object initializer (a thiscall taking the static
/// object plus three scratch words the caller reserves but never fills --
/// verified under the checker's defined zero stack fill, with explicit zero
/// arguments here), then registers the handler and returns the registrar's
/// answer. Both callees are intercepted by the checker.
export!(cdecl, rw_00e5e5e0() -> u32 {
    unsafe {
        /// Static object the initializer sets up (file VA).
        const OBJ: u32 = 0x0110E8E4;
        /// Handler routine this unit registers (file VA).
        const HANDLER: u32 = 0x00E6F0A0;
        let _: u32 = callee_thiscall!(1, u32, relocated(OBJ), 0, 0, 0);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
