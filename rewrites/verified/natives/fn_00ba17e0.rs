// original: 0x00ba17e0 SET_CHAR_MOVE_ANIM_SPEED_MULTIPLIER
/// Script native `SET_CHAR_MOVE_ANIM_SPEED_MULTIPLIER` (hash 0x5DC456DE).
///
/// Char handle + float multiplier bits.
///
/// Float script arguments are forwarded as raw bit patterns,
/// so the forward is bit-exact.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00ba17e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
