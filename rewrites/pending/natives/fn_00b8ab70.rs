// original: 0x00b8ab70 INIT_CUTSCENE
/// Script native `INIT_CUTSCENE` (hash 0x47E50BD3).
///
/// Forwards one script argument (a cutscene name hash) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8ab70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
