// original: 0x00bd97f0 UNOBFUSCATE_INT
/// Script native `UNOBFUSCATE_INT` (hash 0x118D1AA3).
///
/// Forwards two script arguments (an obfuscated value and a key) to the
/// engine. No return slot is written by the handler itself.
export!(cdecl, rw_00bd97f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
