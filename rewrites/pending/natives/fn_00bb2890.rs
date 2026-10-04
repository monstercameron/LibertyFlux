// original: 0x00bb2890 RESET_NUM_OF_MODELS_KILLED_BY_PLAYER
/// Script native `RESET_NUM_OF_MODELS_KILLED_BY_PLAYER` (hash 0x0FB17679).
///
/// Forwards one script word (a player index) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bb2890(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
