// original: 0x00bba060 TASK_OPEN_PASSENGER_DOOR
/// Script native `TASK_OPEN_PASSENGER_DOOR` (hash 0x58F814C4).
///
/// Forwards four script arguments (a character handle, a vehicle handle, a
/// door index and flags) to the engine. No return slot is written.
export!(cdecl, rw_00bba060(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
