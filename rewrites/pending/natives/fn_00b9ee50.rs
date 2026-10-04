// original: 0x00b9ee50 GET_CHAR_HEALTH
/// Script native `GET_CHAR_HEALTH` (hash 0x4B6C2256).
///
/// Forwards two script arguments (a character handle and an out-pointer) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b9ee50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
