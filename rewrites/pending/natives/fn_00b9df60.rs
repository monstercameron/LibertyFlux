// original: 0x00b9df60 ALWAYS_USE_HEAD_ON_HORN_ANIM_WHEN_DEAD_IN_CAR
/// Script native `ALWAYS_USE_HEAD_ON_HORN_ANIM_WHEN_DEAD_IN_CAR` (hash 0x7C156670).
///
/// Turns on/off the need to always use the head on horn animation when dead in a car
///
/// Script arguments: self: Char, use: bool.
///
/// Forwards arg0 (integer/handle), arg1 (boolean flag) to the engine routine.
/// Each flag is coerced with `arg != 0`.
///
/// Quirk (observed): the handler coerces a flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00b9df60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let flag1 = u32::from(*args.add(1) != 0);
        let arg1 = (ctx as u32 & 0xFFFF_FF00) | flag1;
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
