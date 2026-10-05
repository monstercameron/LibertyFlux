// original: 0x00bd9370 SET_NETWORK_JOIN_FAIL
/// Script native `SET_NETWORK_JOIN_FAIL` (hash 0x5849311B).
///
/// Forwards one boolean script argument to the engine. The argument is
/// coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the contract masks
/// that call argument (checks.call_skip). No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd9370(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
