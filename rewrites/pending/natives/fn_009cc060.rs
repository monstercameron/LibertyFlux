// original: 0x009cc060 PLAY_FIRE_SOUND_FROM_POSITION
/// Play a fire sound from a position: pass the four script words (a
/// handle plus three position floats, all bitwise) to the engine. No
/// return value.
export!(cdecl, rw_009cc060(ctx: *const u32) -> u32 {
    unsafe {
        let args = *ctx.add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
        0
    }
});
