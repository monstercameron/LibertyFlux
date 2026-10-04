// original: 0x00b8c420 DISPLAY_TEXT_WITH_TWO_STRINGS
/// Script native `DISPLAY_TEXT_WITH_TWO_STRINGS` (hash 0x66842574).
///
/// Forwards five script arguments to the engine: two float bit-patterns
/// (screen position) followed by a text label and two string references.
/// Floats are copied as raw bits. No return slot is written.
export!(cdecl, rw_00b8c420(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
