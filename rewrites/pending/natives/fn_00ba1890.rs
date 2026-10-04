// original: 0x00ba1890 SET_CHAR_ONLY_DAMAGED_BY_RELATIONSHIP_GROUP
/// Script native `SET_CHAR_ONLY_DAMAGED_BY_RELATIONSHIP_GROUP` (hash 0x506C2898).
///
/// Forwards 3 script argument(s) to the engine: 2 integer(s), a boolean flag.
/// The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00ba1890(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked, *args.add(2))
    }
});
