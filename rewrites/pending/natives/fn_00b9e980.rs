// original: 0x00b9e980 DISABLE_PLAYER_AUTO_VEHICLE_EXIT
/// Script native `DISABLE_PLAYER_AUTO_VEHICLE_EXIT` (hash 0x50E33E8F).
///
/// Disables or re-enables automatic vehicle exit for a player. Forwards the
/// player index and a boolean flag coerced with `arg != 0`.
///
/// Quirk (observed): as in `ALLOW_REACTION_ANIMS`, the flag is coerced into
/// the low byte of the handler's own incoming stack slot, so the pushed
/// dword's high bytes repeat the context pointer; reproduced here exactly.
/// No return slot is written.
export!(cdecl, rw_00b9e980(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
