// original: 0x00b8d980 SET_TEXT_FONT
use lf_k2_rt::{callee_cdecl, export};
/// Native handler `SET_TEXT_FONT`.
///
/// Sets the text draw font.
///
/// Handler mechanics: takes the native call context,
/// Forwards the font index to the engine.
export!(cdecl, rw_00b8d980(ctx: u32) -> () {
    let args = unsafe { *((ctx.wrapping_add(8)) as *const u32) } as *const u32;
    let font = unsafe { *args };
    callee_cdecl!(1, u32, font);
});
