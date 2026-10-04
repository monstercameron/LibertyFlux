// original: 0x00b8ac30 START_CUTSCENE_NOW
/// Script native `START_CUTSCENE_NOW` (hash 0x53591DD7).
///
/// Forwards one script argument (a cutscene name) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8ac30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
