// original: 0x00bb91c0 TASK_CAR_MISSION_COORS_TARGET
/// Script native `TASK_CAR_MISSION_COORS_TARGET` (hash 0x36D51DDF).
///
/// Forwards ten script arguments to the engine: integers and float
/// bit-patterns (target coordinates) copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00bb91c0(ctx: *const u8) -> u32 {
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
        )
    }
});
