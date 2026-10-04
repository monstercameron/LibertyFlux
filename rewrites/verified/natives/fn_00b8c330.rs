// original: 0x00b8c330 DISPLAY_TEXT_WITH_NUMBER
/// Script native `DISPLAY_TEXT_WITH_NUMBER` (hash 0x5A495ABE).
///
/// Forwards four script arguments to the engine: two float bit-patterns
/// (screen position), a text id and a number. No return slot is written.
export!(cdecl, rw_00b8c330(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
