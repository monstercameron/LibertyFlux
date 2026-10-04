// original: 0x00bb1b90 CHANGE_PLAYER_MODEL
/// Script native `CHANGE_PLAYER_MODEL` (hash 0x232F1A85).
///
/// Player index + model hash.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bb1b90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
