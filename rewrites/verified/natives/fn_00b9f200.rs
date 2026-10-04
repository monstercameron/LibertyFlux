// original: 0x00b9f200 GET_DEAD_CHAR_PICKUP_COORDS
/// Script native `GET_DEAD_CHAR_PICKUP_COORDS` (hash 0x2A7475D8).
///
/// Forwards four script arguments (a pickup reference and three
/// out-pointers) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00b9f200(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
