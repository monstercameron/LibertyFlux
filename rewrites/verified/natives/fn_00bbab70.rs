// original: 0x00bbab70 TASK_START_SCENARIO_AT_POSITION
/// Script native `TASK_START_SCENARIO_AT_POSITION` (hash 0x0F296C2E).
///
/// Forwards 6 script arguments (two integers and four coordinate floats) to the engine.
/// Float arguments are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bbab70(ctx: *const u8) -> u32 {
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
        )
    }
});
