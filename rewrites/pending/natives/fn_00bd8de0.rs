// original: 0x00bd8de0 NETWORK_SHOW_MET_PLAYER_FEEDBACK_UI
/// NETWORK_SHOW_MET_PLAYER_FEEDBACK_UI: open the met-player feedback screen.
///
/// Native handler. Forwards the player index to the network UI engine.
export!(cdecl, rw_00bd8de0(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        callee_cdecl!(1, u32, a0)
    }
});
