// original: 0x00bd8cd0 NETWORK_SET_PLAYER_MUTED
/// Script native `NETWORK_SET_PLAYER_MUTED` (hash 0x0B1562DF).
///
/// Forwards two script arguments (a network player index and a boolean flag)
/// to the engine and stores the low byte of its answer (zero-extended)
/// into the return slot. The flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00bd8cd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);

        let answer = callee_cdecl!(1, u32, *args, flag);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
