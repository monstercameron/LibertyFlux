// original: 0x00bb92d0 TASK_CAR_MISSION_PED_TARGET_NOT_AGAINST_TRAFFIC
/// Script native `TASK_CAR_MISSION_PED_TARGET_NOT_AGAINST_TRAFFIC` (hash 0x178332FF).
///
/// Forwards 9 script arguments (eight mixed task arguments and a boolean flag) to the engine.
/// Float arguments are copied as raw bits, so the forward is bit-exact.
/// The flag argument is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag; the argument sits beyond
/// the checker's compared call window so no masking is needed.
/// No return slot is written.
export!(cdecl, rw_00bb92d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(8) != 0);
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
            flag,
        )
    }
});
