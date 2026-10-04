// original: 0x00bb9760 TASK_FALL_AND_GET_UP
/// Script native `TASK_FALL_AND_GET_UP` (hash 0x069433A8).
///
/// Forwards three script arguments (a character handle and task parameters)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bb9760(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
