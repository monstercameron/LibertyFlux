// original: 0x00b9ea30 ENABLE_ALL_PED_HELMETS
/// Script native `ENABLE_ALL_PED_HELMETS` (hash 0x6C305137).
///
/// Forwards one boolean script argument to the engine, coerced with
/// `arg != 0`, using the stack-slot quirk (see
/// `DISPLAY_FRONTEND_MAP_BLIPS`): the pushed dword's high bytes repeat the
/// context pointer. No return slot is written.
/// v2 port: the original repeats context-pointer bytes in this pushed word's high
/// bytes; the rewrite passes the clean flag and the contract masks the argument
/// (checks.call_skip), so only the low byte's behaviour is stated here.
export!(cdecl, rw_00b9ea30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args != 0);
        callee_cdecl!(1, u32, flag)
    }
});
