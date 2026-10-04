// original: 0x00b8d5f0 SET_GPS_REMAINS_WHEN_TARGET_REACHED_FLAG
/// Script native `SET_GPS_REMAINS_WHEN_TARGET_REACHED_FLAG` (hash 0x4C9B749F).
///
/// Forwards one boolean script argument (a boolean flag) to the engine.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00b8d5f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(0) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, quirked)
    }
});
