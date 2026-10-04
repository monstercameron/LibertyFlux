// original: 0x00b8d540 SET_BLIP_COORDINATES
/// Script native `SET_BLIP_COORDINATES` (hash 0x3D91564E).
///
/// Forwards four script arguments (a blip handle and three float bit-patterns for a position) to the engine. The original builds the position through stack temporaries; only the four pushed words are observable. No return slot is written.
export!(cdecl, rw_00b8d540(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
