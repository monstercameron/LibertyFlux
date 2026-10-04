// original: 0x00bc5420 CREATE_CAR_GENERATOR
/// Script native `CREATE_CAR_GENERATOR` (hash 0x0F132F7E).
///
/// Forwards fifteen script arguments to the engine: six floats (coordinates and heading) followed by integers, with argument 11 coerced to a boolean flag. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bc5420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(11) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
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
            *args.add(9),
            *args.add(10),
            quirked,
            *args.add(12),
            *args.add(13),
            *args.add(14),
        )
    }
});
