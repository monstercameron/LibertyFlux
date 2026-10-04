// original: 0x00bba310 TASK_PLAY_ANIM_SECONDARY_NO_INTERRUPT
/// Script native `TASK_PLAY_ANIM_SECONDARY_NO_INTERRUPT` (hash 0x56524B94).
///
/// Forwards nine script arguments to the engine: three integers, one float
/// bit-pattern (moved through an SSE register), and five integers. The
/// float is copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00bba310(ctx: *const u8) -> u32 {
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
