// original: 0x00bc72b0 SET_BIKE_RIDER_WILL_PUT_FOOT_DOWN_WHEN_STOPPED
/// Script native `SET_BIKE_RIDER_WILL_PUT_FOOT_DOWN_WHEN_STOPPED`
/// (hash 0x6E77153D).
///
/// Forwards a ped handle and a boolean flag to the engine. The flag is
/// coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00bc72b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
