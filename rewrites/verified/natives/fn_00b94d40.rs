// original: 0x00b94d40 SET_CLEAR_MANIFOLDS
/// Script native `SET_CLEAR_MANIFOLDS` (hash 0x5B7A738C).
///
/// Forwards one script argument, a boolean flag coerced with `arg != 0`, to the engine. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
export!(cdecl, rw_00b94d40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = flag;
        callee_cdecl!(1, u32, quirked)
    }
});
