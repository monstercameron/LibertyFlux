// original: 0x00bc5370 CREATE_CAR
/// Script native `CREATE_CAR` (hash 0x2F1D6843).
///
/// Forwards 6 script argument(s) to the engine: 3 float bit-pattern(s), 2 integer(s), a boolean flag.
/// The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bc5370(ctx: *const u8) -> u32 {
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
        )
    }
});
