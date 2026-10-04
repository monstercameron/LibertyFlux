// original: 0x00b9ed10 GET_CHAR_ANIM_TOTAL_TIME
/// Script native `GET_CHAR_ANIM_TOTAL_TIME` (hash 0x2E51318F).
///
/// Forwards four script arguments (a character handle, animation names and
/// an out-pointer) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00b9ed10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
