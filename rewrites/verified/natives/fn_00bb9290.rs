// original: 0x00BB9290 TASK_CAR_MISSION_PED_TARGET
// Rewrite of the TASK_CAR_MISSION_PED_TARGET native handler.

/// Script native `TASK_CAR_MISSION_PED_TARGET(...)` (nine arguments).
///
/// Forwards the first eight script arguments unchanged (the fifth travels
/// through a vector register in the original, bits unmodified) plus a boolean
/// coercion of the ninth. The original builds that flag dword by setting the
/// low byte of its own context stack slot, so the high 24 bits repeat the
/// context pointer; that exact dword is reproduced here. No return slot is
/// written; the engine's answer is left in the return register.
export!(cdecl, rw_bb9290(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let flag = ((*args.add(8) != 0) as u32);
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
            flag
        )
    }
});
