// original: 0x00b9ed30 GET_CHAR_AREA_VISIBLE
/// Script native `GET_CHAR_AREA_VISIBLE` (hash 0x06EA1F78).
///
/// Forwards two script arguments (a character handle and a second word) to the engine.
///
/// No return slot is written by the handler itself (observed); any result is delivered through the engine call.
export!(cdecl, rw_00b9ed30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
