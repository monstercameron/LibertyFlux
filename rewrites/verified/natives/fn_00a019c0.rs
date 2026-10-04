// original: 0x00a019c0 SET_ACTIVATE_OBJECT_PHYSICS_AS_SOON_AS_IT_IS_UNFROZEN
/// Script native `SET_ACTIVATE_OBJECT_PHYSICS_AS_SOON_AS_IT_IS_UNFROZEN` (hash 0x378531F8).
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
export!(cdecl, rw_00a019c0(ctx: *const u8) -> u32 {
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
