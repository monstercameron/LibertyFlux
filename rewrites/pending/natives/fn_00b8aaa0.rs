// original: 0x00b8aaa0 CLEAR_NAMED_CUTSCENE
/// Native handler `CLEAR_NAMED_CUTSCENE`.
///
/// Clears the named cutscene.
///
/// Handler mechanics: takes the native call context,
/// Forwards the cutscene name argument to the engine.
lf_rn21_rt::export!(cdecl, rw_00b8aaa0(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let name = unsafe { *args };
    lf_rn21_rt::callee_cdecl!(1, u32, name);
});
