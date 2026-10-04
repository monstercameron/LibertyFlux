// original: 0x00bb89c0 CHANGE_CHAR_SIT_IDLE_ANIM
/// Script native `CHANGE_CHAR_SIT_IDLE_ANIM` (hash 0x7B2822F7).
///
/// Forwards four script arguments (a character handle, an animation-set id and two flags) to the engine. The last argument is coerced with `arg != 0`. No return slot is written.
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00bb89c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(3) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), quirked)
    }
});
