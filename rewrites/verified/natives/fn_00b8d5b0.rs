// original: 0x00b8d5b0 SET_CLEAR_HELP_IN_MISSION_CLEANUP
/// Script native `SET_CLEAR_HELP_IN_MISSION_CLEANUP` (hash 0x4371559F).
///
/// Forwards one boolean flag to the engine. The flag is coerced with
/// `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip). No return slot is written.
export!(cdecl, rw_00b8d5b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(0) != 0);
        callee_cdecl!(1, u32, flag)
    }
});
