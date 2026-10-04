// original: 0x00b8c1e0 DISPLAY_TEXT_WITH_2_NUMBERS
/// Script native `DISPLAY_TEXT_WITH_2_NUMBERS` (hash 0x337957AF).
///
/// Forwards five script arguments to the engine: two float bit-patterns
/// (screen position) followed by a text id and two numbers. Floats travel
/// only as bit patterns, so they are forwarded as words. No return slot is
/// written; the engine answer is the exit value.
export!(cdecl, rw_00b8c1e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4))
    }
});
