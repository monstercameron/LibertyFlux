// original: 0x00bba410 TASK_PLAY_ANIM_WITH_FLAGS
/// Script native `TASK_PLAY_ANIM_WITH_FLAGS` (hash 0x75533E74).
///
/// Forwards six script arguments to the engine: a character handle,
/// /// two string pointers, one float bit-pattern and two integers.
/// /// The float is copied as raw bits, so the forward is bit-exact.
/// /// No return slot is written.
export!(cdecl, rw_00bba410(ctx: *const u8) -> u32 {
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
        )
    }
});
