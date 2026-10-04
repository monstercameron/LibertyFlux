// original: 0x00b8d0f0 PRINT_STRING_WITH_SUBSTRING_GIVEN_HASH_KEY_NOW
/// Script native `PRINT_STRING_WITH_SUBSTRING_GIVEN_HASH_KEY_NOW`
/// (hash 0x00FD3647).
///
/// Forwards four script arguments (a text label, timing and style values) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b8d0f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
