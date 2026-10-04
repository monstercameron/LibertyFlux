// original: 0x00bd0d80 DISABLE_STICKY_BOMB_ACTIVE_SOUND
/// Script native `DISABLE_STICKY_BOMB_ACTIVE_SOUND` (hash 0x0C2D2CC5).
///
/// Forwards two script arguments (a handle and a boolean flag) to the
/// engine. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful. No return slot is written.
export!(cdecl, rw_00bd0d80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
