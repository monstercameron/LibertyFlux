// original: 0x00bd8df0 NETWORK_SHOW_MET_PLAYER_PROFILE_UI
/// Script native `NETWORK_SHOW_MET_PLAYER_PROFILE_UI` (hash 0x1B183AFE).
///
/// Forwards one script argument (a player index) to the engine. No return
/// slot is written. (The original cleans its pushed argument with `pop
/// ecx`; the effect on the stack pointer is identical to the plain cdecl
/// return here.)
export!(cdecl, rw_00bd8df0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
