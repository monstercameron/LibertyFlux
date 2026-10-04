// original: 0x00ba2730 SET_ROOM_FOR_CHAR_BY_NAME
/// Script native `SET_ROOM_FOR_CHAR_BY_NAME` (hash 0x2E9B1F77).
///
/// Forwards a character handle and a room-name hash to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba2730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
