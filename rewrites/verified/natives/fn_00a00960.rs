// original: 0x00a00960 DISABLE_LOCAL_PLAYER_PICKUPS
/// Script native `DISABLE_LOCAL_PLAYER_PICKUPS` (hash 0x19211E9D).
///
/// Forwards one script argument (a boolean flag) to the engine. The flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip).
/// No return slot is written.
export!(cdecl, rw_00a00960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
