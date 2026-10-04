// original: 0x00bb9590 TASK_COMBAT_TIMED
/// Script native `TASK_COMBAT_TIMED` (hash 0x56F04A05).
///
/// Forwards three script arguments (a character handle, a target handle and
/// a duration) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00bb9590(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
