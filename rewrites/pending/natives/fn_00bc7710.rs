// original: 0x00bc7710 SET_CAR_ONLY_DAMAGED_BY_RELATIONSHIP_GROUP
/// Script native `SET_CAR_ONLY_DAMAGED_BY_RELATIONSHIP_GROUP` (hash 0x783F287A).
///
/// Forwards three script arguments (a vehicle handle, a boolean flag
/// coerced with `arg != 0`, and a relationship-group id) to the engine.
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bc7710(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked, *args.add(2))
    }
});
