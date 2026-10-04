// original: 0x00bc7d40 SET_TRAIN_FORCED_TO_SLOW_DOWN
/// Script native `SET_TRAIN_FORCED_TO_SLOW_DOWN` (hash 0x475267B0).
///
/// Forwards a train handle and a boolean flag (`arg != 0`) to the engine.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer; reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00bc7d40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
