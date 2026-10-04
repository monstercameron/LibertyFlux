// original: 0x00a01a60 SET_DOOR_STATE
/// Script native `SET_DOOR_STATE` (hash 0x7E3D3430).
///
/// Forwards three script arguments to the engine: a door handle, a boolean flag coerced with `arg != 0`, and one float bit-pattern. The float is copied as raw bits. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00a01a60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked, *args.add(2))
    }
});
