// original: 0x00b8ce80 LOAD_TEXT_FONT
/// Script native `LOAD_TEXT_FONT` (hash 0x2D371601).
///
/// Forwards one script argument (a font id) to the engine. No return slot
/// is written.
export!(cdecl, rw_00b8ce80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
