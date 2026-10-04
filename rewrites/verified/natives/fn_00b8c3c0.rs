// original: 0x00b8c3c0 DISPLAY_TEXT_WITH_SUBSTRING_GIVEN_HASH_KEY
/// Script native `DISPLAY_TEXT_WITH_SUBSTRING_GIVEN_HASH_KEY` (hash
/// 0x7EF6599D).
///
/// Forwards four script arguments to the engine: two floats (a screen
/// position) copied as raw bits, then two integers (a substring slot and a
/// hash key). No return slot is written.
export!(cdecl, rw_00b8c3c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
