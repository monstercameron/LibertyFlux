// original: 0x00bbaad0 TASK_STAND_GUARD
/// Script native `TASK_STAND_GUARD` (hash 0x59523479).
///
/// Forwards eight script arguments to the engine: a character handle, five float bit-patterns (a position, a heading and a radius), a boolean flag and an integer. The flag is coerced with `arg != 0`. The original shuffles the words through vector registers while building the call frame, but the net effect is a plain in-order forward; floats are copied as raw bits, so the forward is bit-exact.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
/// No return slot is written.
export!(cdecl, rw_00bbaad0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(6) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), quirked, *args.add(7),)
    }
});
