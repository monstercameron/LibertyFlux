// original: 0x00bc7ad0 SET_NO_RESPRAYS
/// Script native `SET_NO_RESPRAYS` (hash 0x418D0889).
///
/// Forwards one boolean script argument to the engine, coerced with `arg !=
/// 0`. No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred). The rewrite passes the clean 0/1 flag; the pushed
/// word carries context-pointer high bytes on the original side, so
/// the contract skips that call argument (v2 has no low-byte compare).
export!(cdecl, rw_00bc7ad0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);

        callee_cdecl!(1, u32, flag)
    }
});
