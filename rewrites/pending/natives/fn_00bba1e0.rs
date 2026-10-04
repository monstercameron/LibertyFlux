// original: 0x00bba1e0 TASK_PLAY_ANIM_NON_INTERRUPTABLE
/// Script native `TASK_PLAY_ANIM_NON_INTERRUPTABLE` (hash 0x52202E76).
///
/// Forwards nine script arguments to the engine: integers and one float
/// bit-pattern (the fourth word). No return slot is written.
export!(cdecl, rw_00bba1e0(ctx: *const u8) -> u32 {
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
