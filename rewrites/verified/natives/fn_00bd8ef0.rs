// original: 0x00bd8ef0 OBFUSCATE_INT
/// Script native `OBFUSCATE_INT` (hash 0x31A219FA).
///
/// Forwards two script arguments to the engine. No return slot is written
/// by the handler itself.
export!(cdecl, rw_00bd8ef0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
