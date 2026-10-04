// original: 0x00ba1660 SET_CHAR_MAX_HEALTH
/// Script native `SET_CHAR_MAX_HEALTH` (hash 0x08A453C9).
///
/// Forwards 2 script arguments (a character handle and a health value) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba1660(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
