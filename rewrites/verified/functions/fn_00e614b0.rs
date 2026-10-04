// original: 0x00e614b0 timing_slot2_init_and_register
/// Initialise timer child slot 2, then register its callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e614b0() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E70120))
    }
});
