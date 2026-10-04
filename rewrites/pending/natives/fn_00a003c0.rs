// original: 0x00a003c0 ADD_PICKUP_TO_INTERIOR_ROOM_BY_KEY
/// Script native `ADD_PICKUP_TO_INTERIOR_ROOM_BY_KEY` (hash 0x198B786F).
///
/// Forwards two script arguments (a pickup handle and a room key) to the engine. No return slot is written.
export!(cdecl, rw_00a003c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
