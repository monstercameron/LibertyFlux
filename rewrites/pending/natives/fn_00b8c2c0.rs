// original: 0x00b8c2c0 DISPLAY_TEXT_WITH_LITERAL_STRING
/// Script native `DISPLAY_TEXT_WITH_LITERAL_STRING` (hash 0x661B239A).
///
/// Forwards four script arguments to the engine: two float bit-patterns
/// (text position) and two integers (string reference and style). Floats
/// are copied as raw bits, so the forward is bit-exact. No return slot
/// is written.
export!(cdecl, rw_00b8c2c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
