// original: 0x00bb9210 TASK_CAR_MISSION_COORS_TARGET_NOT_AGAINST_TRAFFIC
/// Script native `TASK_CAR_MISSION_COORS_TARGET_NOT_AGAINST_TRAFFIC`
/// (hash 0x3CB4693B).
///
/// Forwards ten script arguments to the engine: two integers, three float
/// bit-patterns, one integer, one float bit-pattern and three integers.
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00bb9210(ctx: *const u8) -> u32 {
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
