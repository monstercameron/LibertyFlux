// original: 0x00bbac70 TASK_TOGGLE_DUCK
/// Script native `TASK_TOGGLE_DUCK` (hash 0x319E3A87).
///
/// Forwards two script arguments (a character handle and a toggle flag) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bbac70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
