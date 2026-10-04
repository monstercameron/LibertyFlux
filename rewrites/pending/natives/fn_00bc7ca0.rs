// original: 0x00bc7ca0 SET_ROOM_FOR_CAR_BY_KEY
/// Script native `SET_ROOM_FOR_CAR_BY_KEY`.
///
/// Forwards two script arguments (a vehicle handle and an interior-room
/// key) to the engine. No return slot is written.
export!(cdecl, rw_00bc7ca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
