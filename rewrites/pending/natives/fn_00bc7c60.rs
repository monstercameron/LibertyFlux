// original: 0x00bc7c60 SET_RENDER_TRAIN_AS_DERAILED
/// Script native `SET_RENDER_TRAIN_AS_DERAILED` (hash 0x08240FB7).
///
/// Forwards 2 script arguments (a vehicle handle and a boolean flag) to the engine.
/// The flag argument is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00bc7c60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
