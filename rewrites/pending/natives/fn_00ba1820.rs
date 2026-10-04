// original: 0x00ba1820 SET_CHAR_NEVER_TARGETTED
/// Script native `SET_CHAR_NEVER_TARGETTED` (hash 0x5EA84115).
///
/// Forwards two script arguments (a character handle and a flag) to the
/// engine; the flag is coerced to 0/1. No return slot is written.
///
/// Quirk (observed): the flag's low byte is set into the handler's own
/// incoming stack slot and the whole dword is pushed, so its high bytes
/// repeat the context pointer. Reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00ba1820(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
