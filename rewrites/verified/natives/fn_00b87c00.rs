// original: 0x00b87c00 SET_ROOM_FOR_VIEWPORT_BY_KEY
/// Script native `SET_ROOM_FOR_VIEWPORT_BY_KEY` (hash 0x07EE2A45).
///
/// Forwards two script arguments (a viewport handle and a room key) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b87c00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
