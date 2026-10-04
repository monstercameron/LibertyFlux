// original: 0x00b9ee30 GET_CHAR_HEADING
/// Script native `GET_CHAR_HEADING` (hash 0x057A3AC7).
///
/// Forwards two script arguments (a character handle and a second word)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b9ee30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
