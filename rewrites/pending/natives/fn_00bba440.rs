// original: 0x00bba440 TASK_PLAY_ANIM_WITH_FLAGS_AND_START_PHASE
/// Script native `TASK_PLAY_ANIM_WITH_FLAGS_AND_START_PHASE` (hash 0x1A122D03).
///
/// Forwards seven script arguments (a ped handle, animation selectors and two float parameters, forwarded as raw bits) to the engine animation task.
/// No return slot is written.
export!(cdecl, rw_00bba440(ctx: *const u8) -> u32 {
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
        )
    }
});
