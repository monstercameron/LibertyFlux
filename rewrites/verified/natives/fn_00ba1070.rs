// original: 0x00ba1070 SET_CHAR_AS_MISSION_CHAR
/// Script native `SET_CHAR_AS_MISSION_CHAR` (hash 0x60EC0540).
///
/// Forwards one character handle to the engine. Writes no return slot.
export!(cdecl, rw_00ba1070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0))
    }
});
