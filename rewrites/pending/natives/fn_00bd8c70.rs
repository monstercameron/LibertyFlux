// original: 0x00bd8c70 NETWORK_SET_LOCAL_PLAYER_CAN_TALK
/// Script native `NETWORK_SET_LOCAL_PLAYER_CAN_TALK` (hash 0x4CC379D0).
///
/// Forwards two script arguments (a player index and a boolean flag
/// coerced with `arg != 0`) to the engine and stores the low byte of its
/// answer (zero-extended) into the return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00bd8c70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);

        let answer = callee_cdecl!(1, u32, *args, flag);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
