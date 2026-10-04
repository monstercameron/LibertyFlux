// original: 0x00b9f320 GET_KEY_FOR_CHAR_IN_ROOM
/// Script native `GET_KEY_FOR_CHAR_IN_ROOM` (hash 0x266D0801).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b9f320(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
