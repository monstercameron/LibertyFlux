// original: 0x00bb1c50 CLEAR_WANTED_LEVEL
/// Script native `CLEAR_WANTED_LEVEL` (hash 0x205622AC).
///
/// Clears the player's wanted level
///
/// Script arguments: self: Player.
///
/// Forwards arg0 (integer/handle) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00bb1c50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        callee_cdecl!(1, u32, arg0, )
    }
});
