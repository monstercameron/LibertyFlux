// original: 0x00bb2000 GET_PLAYER_WANTED_LEVEL_INCREMENT
/// Read a player's wanted-level increment into an out-parameter.
///
/// Forwards the player index (argument 0) and the out-pointer (argument 1) to
/// the engine implementation. Returns whatever the engine call returned.
export!(cdecl, rw_00bb2000(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
