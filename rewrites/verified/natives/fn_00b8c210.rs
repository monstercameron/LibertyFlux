// original: 0x00b8c210 DISPLAY_TEXT_WITH_3_NUMBERS
/// Script native `DISPLAY_TEXT_WITH_3_NUMBERS` (hash 0x746C06E8).
///
/// Forwards six script arguments to the engine: two float bit-patterns
/// (screen position) followed by a text id and three integer numbers.
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00b8c210(ctx: *const u8) -> u32 {
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
            *args.add(5),
        )
    }
});
