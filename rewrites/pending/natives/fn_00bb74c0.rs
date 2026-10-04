// original: 0x00bb74c0 LOAD_SCENE_FOR_ROOM_BY_KEY
/// Script native `LOAD_SCENE_FOR_ROOM_BY_KEY` (hash 0x6E904C1A).
///
/// Forwards two script arguments (room key parts) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb74c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
