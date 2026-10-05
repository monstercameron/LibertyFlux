// original: 0x00bba220 TASK_PLAY_ANIM_ON_CLONE
/// Script native `TASK_PLAY_ANIM_ON_CLONE` (hash 0x10FB7B5F).
///
/// Tasks a ped clone with playing an animation. Forwards nine script
/// arguments to the engine, one of which (the blend rate) is a float copied
/// as raw bits. No return slot is written.
///
/// (The original stages that float through a `(an instruction of the original)` slot, but overwrites
/// the whole dword before the call, so no caller-register garbage survives
/// into the observed behaviour.)
export!(cdecl, rw_00bba220(ctx: *const u8) -> u32 {
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
