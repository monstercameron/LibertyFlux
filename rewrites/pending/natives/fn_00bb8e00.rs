// original: 0x00bb8e00 MODIFY_CHAR_MOVE_STATE
/// Script native `MODIFY_CHAR_MOVE_STATE` (hash 0x5CD32071).
///
/// Forwards two script arguments (a character handle and a move state) to
/// the engine. No return slot is written.
export!(cdecl, rw_00bb8e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
