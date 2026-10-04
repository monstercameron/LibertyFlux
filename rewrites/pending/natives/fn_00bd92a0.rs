// original: 0x00bd92a0 SET_IN_SPECTATOR_MODE
/// Script native `SET_IN_SPECTATOR_MODE` (hash 0x40035D5D).
///
/// Forwards one boolean script argument to the engine. The flag is coerced with `arg != 0`.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
/// No return slot is written.
export!(cdecl, rw_00bd92a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);

        callee_cdecl!(1, u32, flag,)
    }
});
