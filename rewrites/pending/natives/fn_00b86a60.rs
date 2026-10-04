// original: 0x00b86a60 ENABLE_CAM_COLLISION
/// Script native `ENABLE_CAM_COLLISION` (hash 0x71AE1BDC).
///
/// Forwards a camera handle and a boolean flag (coerced with `arg != 0`)
/// to the engine.
///
/// Quirk (observed): the flag is coerced into the low byte of the
/// handler's own incoming stack slot and the whole dword is pushed, so its
/// high bytes repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b86a60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
