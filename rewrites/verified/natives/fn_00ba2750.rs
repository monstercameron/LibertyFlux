// original: 0x00ba2750 SET_ROOM_FOR_DUMMY_CHAR_BY_KEY
/// Script native `SET_ROOM_FOR_DUMMY_CHAR_BY_KEY` (hash 0x29907BEF).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00ba2750(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
