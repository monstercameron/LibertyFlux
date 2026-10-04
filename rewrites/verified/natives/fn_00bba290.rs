// original: 0x00bba290 TASK_PLAY_ANIM_SECONDARY
/// Script native `TASK_PLAY_ANIM_SECONDARY` (hash 0x273C2D35).
///
/// Forwards nine script arguments (handles, a float rate and flag words) to the engine. No return slot is written.
export!(cdecl, rw_00bba290(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8))
    }
});
