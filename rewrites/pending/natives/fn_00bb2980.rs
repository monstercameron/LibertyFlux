// original: 0x00bb2980 SET_PLAYERSETTINGS_MODEL_VARIATIONS_CHOICE
/// Script native `SET_PLAYERSETTINGS_MODEL_VARIATIONS_CHOICE`
/// (hash 0x27650F37).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bb2980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
