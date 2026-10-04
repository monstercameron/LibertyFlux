// original: 0x00bc62a0 GET_RANDOM_CAR_IN_SPHERE_NO_SAVE
/// Script native `GET_RANDOM_CAR_IN_SPHERE_NO_SAVE` (hash 0x0A7E36E5).
///
/// Forwards seven script arguments to the engine: four float bit-patterns
/// (sphere centre and radius), an integer, a boolean flag coerced with
/// `arg != 0`, and a final integer. Floats are copied as raw bits, so the
/// forward is bit-exact.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00bc62a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(5) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            quirked,
            *args.add(6),
        )
    }
});
