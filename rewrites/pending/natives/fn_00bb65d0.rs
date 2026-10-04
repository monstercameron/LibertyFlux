// original: 0x00bb65d0 REGISTER_MISSION_PASSED
/// Native handler `REGISTER_MISSION_PASSED`.
///
/// Sets the GXT entry of the last mission passed.
///
/// Handler mechanics: takes the native call context,
/// Forwards the GXT argument to the engine.
lf_rn21_rt::export!(cdecl, rw_00bb65d0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let gxt = unsafe { *args };
    lf_rn21_rt::callee_cdecl!(1, u32, gxt);
});
