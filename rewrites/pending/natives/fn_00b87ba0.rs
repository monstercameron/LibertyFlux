// original: 0x00b87ba0 SET_INTERP_FROM_GAME_TO_SCRIPT
/// Script native `SET_INTERP_FROM_GAME_TO_SCRIPT` (hash 0x45CE21CA).
///
/// Forwards a boolean flag (`arg0 != 0`) and a second script argument to
/// the engine. No return slot is written.
///
/// Quirk (observed): the handler coerces the first argument into the low
/// byte of its own incoming stack slot and pushes the whole dword, so the
/// pushed word's high bytes repeat the context pointer. The engine reads
/// only the low byte (Inferred). The rewrite passes the clean 0/1
/// flag; the contract skips that call argument (v2 has no low-byte
/// compare) since the original's pushed word carries context high bytes.
export!(cdecl, rw_00b87ba0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);

        callee_cdecl!(1, u32, flag, *args.add(1))
    }
});
