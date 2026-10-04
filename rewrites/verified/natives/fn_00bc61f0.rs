// original: 0x00bc61f0 GET_RANDOM_CAR_FRONT_BUMPER_IN_SPHERE_NO_SAVE
/// Script native `GET_RANDOM_CAR_FRONT_BUMPER_IN_SPHERE_NO_SAVE` (hash 0x13C91ACD).
///
/// Forwards eight script arguments (four float bit-patterns, an integer, a boolean flag and two integers) to the engine. No return slot is written. The boolean flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
export!(cdecl, rw_00bc61f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag5 = u32::from(*args.add(5) != 0);
        let quirked5 = flag5;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            quirked5,
            *args.add(6),
            *args.add(7),
        )
    }
});
