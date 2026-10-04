// original: 0x00bd8f10 OBFUSCATE_INT_ARRAY
/// Script native `OBFUSCATE_INT_ARRAY` (hash 0x3EF15B6A).
///
/// Forwards two script arguments (an array pointer and a count) to the engine. No return slot is written.
export!(cdecl, rw_00bd8f10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
