// original: 0x00bc5df0 GET_DEAD_CAR_COORDINATES
/// Script native `GET_DEAD_CAR_COORDINATES` (hash 0x3BC827E6).
///
/// Forwards four script arguments (a vehicle handle and three out-pointers)
/// to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bc5df0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
