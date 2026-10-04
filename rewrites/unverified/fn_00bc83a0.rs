// original: 0x00bc83a0 VEHICLE_DOES_PROVIDE_COVER
/// Script native `VEHICLE_DOES_PROVIDE_COVER` (hash 0x0C4F5021).
///
/// Forwards a vehicle handle and a boolean flag to the engine.
/// The flag is coerced with `arg != 0`.
/// No return slot is written.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bc83a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked,)
    }
});
