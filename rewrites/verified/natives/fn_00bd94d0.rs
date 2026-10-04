// original: 0x00bd94d0 SET_PLAYER_AS_DAMAGED_PLAYER
/// Script native `SET_PLAYER_AS_DAMAGED_PLAYER` (hash 0x633A012B).
///
/// Forwards 3 script arguments (three script arguments) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd94d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});

