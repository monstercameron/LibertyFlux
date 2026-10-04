// original: 0x00bb1a90 ALLOW_LOCKON_TO_FRIENDLY_PLAYERS
/// Forward (player, allow) to the lock-on engine function.
///
/// The flag argument is coerced to 0/1 by testing the whole dword; the
/// original does this by overwriting the low byte of its own incoming stack
/// slot, so the pushed dword keeps the context address in its high bytes.
/// That value is reproduced exactly (the engine reads only the low byte).
rt::export!(cdecl, rw_00bb1a90(ctx: u32) -> u32 {
    const RET_SLOT: usize = 0;
    const ARG_ARRAY: usize = 2;
    let _ = RET_SLOT;
    let c = ctx as *const u32;
    let args = unsafe { *c.add(ARG_ARRAY) } as *const u32;
    let player = unsafe { *args.add(0) };
    let allow = unsafe { *args.add(1) };
    let coerced = (ctx & 0xFFFF_FF00) | ((allow != 0) as u32);
    rt::callee_cdecl!(1, u32, player, coerced)
});
