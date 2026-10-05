// original: 0x00b87d20 SET_TIME_CYCLE_FAR_CLIP_DISABLED
/// Script native `SET_TIME_CYCLE_FAR_CLIP_DISABLED` (hash 0x13C75E16).
///
/// Forwards one boolean script argument to the engine. The argument is coerced with `arg != 0`. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)

/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00b87d20(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
