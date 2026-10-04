// original: 0x00bb1b10 ALTER_WANTED_LEVEL_NO_DROP
/// Script native `ALTER_WANTED_LEVEL_NO_DROP` (hash 0x5F3B6079).
///
/// Forwards two script arguments (a player index and a wanted level) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb1b10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1),)
    }
});
