// original: 0x00b8da10 SET_TEXT_RENDER_ID
/// Native handler `SET_TEXT_RENDER_ID`.
///
/// Selects the text render target.
///
/// Handler mechanics: takes the native call context,
/// Forwards the render id to the engine.
lf_rn21_rt::export!(cdecl, rw_00b8da10(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let id = unsafe { *args };
    lf_rn21_rt::callee_cdecl!(1, u32, id);
});
