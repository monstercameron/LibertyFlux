// original: 0x00b9edb0 GET_CHAR_EXTRACTED_DISPLACEMENT
/// Script native `GET_CHAR_EXTRACTED_DISPLACEMENT` (hash 0x466B5AA0).
///
/// Forwards five script arguments to the engine: a character handle, a
/// boolean flag, and three further words. The flag is coerced with
/// `arg != 0`.
///
/// Quirk (observed): the handler coerces the flag into the low byte of its
/// own incoming stack slot and pushes the whole dword, so the pushed word's
/// high bytes repeat the context pointer. The engine reads only the low
/// byte (Inferred); the full dword is reproduced here for bit-exact
/// outgoing-call matching. No return slot is written.
export!(cdecl, rw_00b9edb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(
            1,
            u32,
            *args,
            quirked,
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
