// original: 0x00ba1dd0 SET_DEAD_CHAR_COORDINATES
/// Script native `SET_DEAD_CHAR_COORDINATES` (hash 0x68C57282).
///
/// Forwards four script arguments (a character handle and three float
/// bit-patterns for a position) to the engine worker.
/// No return slot is written.
export!(cdecl, rw_00ba1dd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
