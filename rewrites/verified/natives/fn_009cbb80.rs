// original: 0x009cbb80 FORCE_INITIAL_PLAYER_STATION
/// Script native `FORCE_INITIAL_PLAYER_STATION` (hash 0x32D3165D).
///
/// Forwards one script argument (a station id) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cbb80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
