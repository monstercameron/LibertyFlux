// original: 0x00b8c360 DISPLAY_TEXT_WITH_STRING
/// Script native `DISPLAY_TEXT_WITH_STRING` (hash 0x10A75905).
///
/// Forwards four script arguments (two float coordinates, a text id and a string reference) to the engine. No return slot is written.
export!(cdecl, rw_00b8c360(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
