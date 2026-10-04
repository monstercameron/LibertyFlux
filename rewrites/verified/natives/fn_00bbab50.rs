// original: 0x00bbab50 TASK_STAND_STILL
/// Script native `TASK_STAND_STILL` (hash 0x524C4CB5).
///
/// Forwards two script arguments (a character handle and a duration) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bbab50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
