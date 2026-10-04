// original: 0x00bb90a0 TASK_CAR_DRIVE_TO_COORD
/// Script native `TASK_CAR_DRIVE_TO_COORD` (hash 0x69715285).
///
/// Forwards eleven script arguments to the engine: handles, integers and
/// five float bit-patterns (coordinates and driving parameters) copied
/// unchanged. No return slot is written.
export!(cdecl, rw_00bb90a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
            *args.add(7),
            *args.add(8),
            *args.add(9),
            *args.add(10)
        )
    }
});
