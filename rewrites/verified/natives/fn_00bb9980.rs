// original: 0x00bb9980 TASK_FOLLOW_POINT_ROUTE
/// Script native `TASK_FOLLOW_POINT_ROUTE` (hash 0x1C430F41).
///
/// Forwards three script arguments (a character handle and route parameters)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bb9980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
