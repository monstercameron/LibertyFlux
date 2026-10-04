// original: 0x00b8c680 GET_BLIP_COLOUR
/// Script native `GET_BLIP_COLOUR` (hash 0x59B425DA).
///
/// Forwards two script arguments (a blip handle and a second word) to the engine.
///
/// No return slot is written by the handler itself (observed); any result is delivered through the engine call.
export!(cdecl, rw_00b8c680(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
