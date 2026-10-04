// original: 0x00bc7960 SET_FREE_RESPRAYS
/// Script native `SET_FREE_RESPRAYS` (hash 0x00710A49).
///
/// Forwards one boolean script argument to the engine. The flag is coerced
/// with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip). No return slot is written.
export!(cdecl, rw_00bc7960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, quirked)
    }
});
