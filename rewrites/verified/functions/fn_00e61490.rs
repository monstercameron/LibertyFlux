// original: 0x00e61490 timing_slot1_init_and_register
/// Initialise timer child slot 1, then register its callback.
///
/// Returns the registrar's answer.
export!(cdecl, rw_00e61490() -> u32 {
    unsafe {
        callee_cdecl!(1, u32,);
        callee_cdecl!(2, u32, relocated(0x00E70110))
    }
});
