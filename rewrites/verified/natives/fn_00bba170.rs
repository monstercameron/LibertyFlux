// original: 0x00bba170 TASK_PLAY_ANIM
/// Script native `TASK_PLAY_ANIM` (hash 0x28EE78D8).
///
/// Forwards nine script arguments (a character handle, animation names, one
/// float bit-pattern for the speed, and flag integers) to the engine. The
/// float is copied as raw bits, so the forward is bit-exact. No return slot
/// is written.
export!(cdecl, rw_00bba170(ctx: *const u8) -> u32 {
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
