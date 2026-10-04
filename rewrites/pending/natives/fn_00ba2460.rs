// original: 0x00ba2460 SET_PED_GENERATES_DEAD_BODY_EVENTS
/// Script native `SET_PED_GENERATES_DEAD_BODY_EVENTS` (hash 0x3DBF53E0).
///
/// Forwards 2 script arguments to the engine in order.
///
/// Quirk (observed): the handler coerces argument 1 with
/// `arg != 0` into the low byte of its own incoming stack slot
/// and pushes the whole dword, so the pushed word's high bytes
/// repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for
/// bit-exact outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00ba2460(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            quirked,
        );
        answer
    }
});
