// original: 0x00bba3d0 TASK_PLAY_ANIM_WITH_ADVANCED_FLAGS
/// Script native `TASK_PLAY_ANIM_WITH_ADVANCED_FLAGS`.
///
/// Forwards twelve script arguments to the engine: handles, animation
/// names, flag words and one float bit-pattern (the playback rate at
/// script word 3). The float is copied as raw bits, so the forward is
/// bit-exact. No return slot is written.
export!(cdecl, rw_00bba3d0(ctx: *const u8) -> u32 {
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
            *args.add(7),
            *args.add(8),
            *args.add(9),
            *args.add(10),
            *args.add(11),
        )
    }
});
