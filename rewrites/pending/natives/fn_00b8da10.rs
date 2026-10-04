// original: 0x00b8da10 SET_TEXT_RENDER_ID
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_TEXT_RENDER_ID`.
///
/// Selects the text render target.
///
/// Handler mechanics: takes the native call context,
/// Forwards the render id to the engine.
export!(cdecl, rw_00b8da10(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let id = unsafe { *args };
    callee_cdecl!(1, u32, id);
});
