// original: 0x00b8c3f0 DISPLAY_TEXT_WITH_TWO_LITERAL_STRINGS
/// Script native `DISPLAY_TEXT_WITH_TWO_LITERAL_STRINGS` (hash 0x4B7C3AEC).
///
/// Forwards five script arguments to the engine: two float coordinates as
/// raw bits followed by three integers (literal-string references and a
/// text id). No return slot is written.
export!(cdecl, rw_00b8c3f0(ctx: *const u8) -> u32 {
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
