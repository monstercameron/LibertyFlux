// original: 0x009cb8e0 ABORT_SCRIPTED_CONVERSATION
/// Script native `ABORT_SCRIPTED_CONVERSATION` (hash 0x57DB70CE).
///
/// Forwards one boolean script argument (coerced with `arg != 0`) to the
/// engine and stores the engine's full 32-bit answer into the return slot.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_009cb8e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let answer = callee_cdecl!(1, u32, flag);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
