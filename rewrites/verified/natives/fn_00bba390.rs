// original: 0x00bba390 TASK_PLAY_ANIM_UPPER_BODY
/// Script native `TASK_PLAY_ANIM_UPPER_BODY` (hash 0x02534709).
///
/// Forwards nine script arguments to the engine: two handles, one float
/// bit-pattern (playback speed) and six trailing option words. No return
/// slot is written.
export!(cdecl, rw_00bba390(ctx: *const u8) -> u32 {
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
        )
    }
});
