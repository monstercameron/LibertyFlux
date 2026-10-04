// original: 0x00b8bce0 CHANGE_BLIP_NAME_FROM_ASCII
/// Script native `CHANGE_BLIP_NAME_FROM_ASCII` (hash 0x6C9F2330).
///
/// Renames a blip from an ASCII string. Forwards two script arguments (the
/// blip handle and the string pointer) to the engine. No return slot is
/// written.
export!(cdecl, rw_00b8bce0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
