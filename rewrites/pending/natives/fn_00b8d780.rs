// original: 0x00b8d780 SET_MISSION_PASSED_CASH
/// Script native `SET_MISSION_PASSED_CASH` (hash 0x60DC6E25).
///
/// Forwards three script arguments to the engine: a boolean flag
/// coerced with `arg != 0` followed by two integers.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b8d780(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked, *args.add(1), *args.add(2))
    }
});
