// original: 0x00ba2050 SET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT
/// Script native `SET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT` (hash 0x5C8C7F9E).
///
/// Forwards a group handle and a bool flag to the engine. Quirk
/// (observed): the flag dword's high bytes repeat the context pointer (the
/// handler coerces into its own stack slot); reproduced here for bit-exact
/// outgoing-call matching.
export!(cdecl, rw_00ba2050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let coerced = (ctx as u32 & 0xFFFF_FF00) | u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, coerced)
    }
});
