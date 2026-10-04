// original: 0x00b8c250 DISPLAY_TEXT_WITH_BLIP_NAME
/// Script native `DISPLAY_TEXT_WITH_BLIP_NAME` (hash 0x7E8D1DCE).
///
/// Forwards four script arguments to the engine: two float bit-patterns
/// (screen position) followed by two integers (text identifiers). Floats
/// are copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00b8c250(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
