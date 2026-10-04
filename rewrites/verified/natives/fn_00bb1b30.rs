// original: 0x00bb1b30 APPLY_WANTED_LEVEL_CHANGE_NOW
/// Script native `APPLY_WANTED_LEVEL_CHANGE_NOW` (hash 0x705A6ED9).
///
/// Forwards one script argument (a player index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb1b30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
