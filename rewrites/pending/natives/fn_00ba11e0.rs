// original: 0x00ba11e0 SET_CHAR_COORDINATES_DONT_CLEAR_PLAYER_TASKS
/// Native handler `SET_CHAR_COORDINATES_DONT_CLEAR_PLAYER_TASKS`.
///
/// Move a character to coordinates without clearing player tasks.
/// Forwards 4 argument(s) to the engine and returns nothing.
export!(cdecl, rw_00ba11e0(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let arg2 = *args.add(2);
        let arg3 = *args.add(3);
        let _ = arg0;
        callee_cdecl!(1, u32, arg0, arg1, arg2, arg3);
        0
    }
});
