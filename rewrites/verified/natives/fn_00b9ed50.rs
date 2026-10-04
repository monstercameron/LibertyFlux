// original: 0x00b9ed50 GET_CHAR_ARMOUR
/// Script native `GET_CHAR_ARMOUR` (hash 0x3C756E54).
///
/// Forwards two script arguments (a character handle and an out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b9ed50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
