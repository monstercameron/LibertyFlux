// original: 0x00bb9960 TASK_FOLLOW_PATROL_ROUTE
/// Script native `TASK_FOLLOW_PATROL_ROUTE` (hash 0x72F02B67).
///
/// Forwards three script arguments (a character handle, a patrol-route id and flags) to the engine. No return slot is written.
export!(cdecl, rw_00bb9960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
