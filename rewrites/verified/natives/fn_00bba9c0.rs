// original: 0x00bba9c0 TASK_SMART_FLEE_CHAR_PREFERRING_PAVEMENTS
/// Script native `TASK_SMART_FLEE_CHAR_PREFERRING_PAVEMENTS`
/// (hash 0x57AC66E9).
///
/// Tasks a character with fleeing from another, preferring pavements.
/// Forwards four script arguments to the engine, one of which (the flee
/// radius) is a float copied as raw bits. No return slot is written.
///
/// (The original stages that float through a `(an instruction of the original)` slot, but overwrites
/// the whole dword before the call, so no caller-register garbage survives
/// into the observed behaviour.)
export!(cdecl, rw_00bba9c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
