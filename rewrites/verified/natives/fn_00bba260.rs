// original: 0x00bba260 TASK_PLAY_ANIM_READY_TO_BE_EXECUTED
/// Script native `TASK_PLAY_ANIM_READY_TO_BE_EXECUTED` (hash 0x040A0537).
///
/// Forwards three integers and one float bit-pattern to the engine.
/// No return slot is written.
export!(cdecl, rw_00bba260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
