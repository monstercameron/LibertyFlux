// original: 0x00ba1020 SET_CHAR_ANIM_SPEED
/// Script native `SET_CHAR_ANIM_SPEED` (hash 0x3C2A3334).
///
/// Forwards four script arguments (a character handle, two ids and a float speed, copied as raw bits) to the engine. No return slot is written.
export!(cdecl, rw_00ba1020(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3))
    }
});
