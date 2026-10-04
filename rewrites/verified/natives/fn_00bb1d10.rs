// original: 0x00bb1d10 DISABLE_PLAYER_VEHICLE_ENTRY
/// Script native `DISABLE_PLAYER_VEHICLE_ENTRY` (hash 0x05D51783).
///
/// Forwards two script arguments (a player index and a boolean flag) to the engine. The flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip).
/// No return slot is written.
export!(cdecl, rw_00bb1d10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
