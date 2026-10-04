// original: 0x00bd9540 SET_RICH_PRESENCE_TEMPLATELOBBY
/// Script native `SET_RICH_PRESENCE_TEMPLATELOBBY` (hash 0x77D72045).
///
/// Forwards one script argument (a template value) to the engine.
///
/// No return slot is written.
export!(cdecl, rw_00bd9540(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
