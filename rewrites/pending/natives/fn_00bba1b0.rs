// original: 0x00bba1b0 TASK_PLAY_ANIM_FACIAL
/// Script native `TASK_PLAY_ANIM_FACIAL` (hash 0x71F001D2).
///
/// Forwards seven script arguments to the engine: three integers, one
/// float bit-pattern (blend speed), and three more integers. The float is
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bba1b0(ctx: *const u8) -> u32 {
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
