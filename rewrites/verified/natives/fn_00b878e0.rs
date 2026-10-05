// original: 0x00b878e0 SET_ENABLE_NEAR_CLIP_SCAN
/// Script native `SET_ENABLE_NEAR_CLIP_SCAN` (hash 0x35CC3267).
///
/// Forwards one boolean script argument to the engine. No return slot is written. The boolean flag is coerced with `arg != 0`.

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the clean flag and the pushed
/// argument is masked in the contract (call_skip).
///
/// The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.
export!(cdecl, rw_00b878e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag0 = u32::from(*args != 0);
        let quirked0 = flag0;
        callee_cdecl!(1, u32, quirked0)
    }
});
