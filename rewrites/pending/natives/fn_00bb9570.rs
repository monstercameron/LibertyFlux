// original: 0x00bb9570 TASK_COMBAT_ROLL
/// Script native `TASK_COMBAT_ROLL` (hash 0x131A0C84).
///
/// Forwards a character handle and a direction flag to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb9570(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
