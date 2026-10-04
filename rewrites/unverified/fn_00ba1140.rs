// original: 0x00ba1140 SET_CHAR_CLIMB_ANIM_RATE
/// Script native `SET_CHAR_CLIMB_ANIM_RATE` (hash 0x68AB2DD9).
///
/// Forwards a character handle and a float bit-pattern (the rate) to the
/// engine. No return slot is written.
export!(cdecl, rw_00ba1140(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
