// original: 0x00e61530 timing_slot6_init_and_register
/// Initialise timer child slot 6, then register its callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e61530() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E70160))
    }
});
