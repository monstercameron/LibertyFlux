// original: 0x00bb2de0 STORE_WANTED_LEVEL
/// Script native `STORE_WANTED_LEVEL` (hash 0x12AA6D71).
///
/// Forwards 2 script arguments (a player index and an out-pointer) to the engine.
///
/// No return slot is written.
///
export!(cdecl, rw_00bb2de0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
