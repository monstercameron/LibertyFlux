// original: 0x00a02000 SET_PLAYER_CAN_DROP_WEAPONS_IN_CAR
/// Script native `SET_PLAYER_CAN_DROP_WEAPONS_IN_CAR` (hash 0x4F884E33).
///
/// Forwards one boolean script argument (coerced with `arg != 0`) to the
/// engine. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00a02000(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
