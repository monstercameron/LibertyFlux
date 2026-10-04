// original: 0x00b944d0 GET_GAME_TIMER
/// Script native `GET_GAME_TIMER` (hash 0x022B2DA9).
///
/// Forwards one script argument to the engine. The handler itself writes no
/// return slot (any result travels through the engine call). No return slot
/// is written here.
export!(cdecl, rw_00b944d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
