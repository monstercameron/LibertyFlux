// original: 0x00ba2050 SET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT
/// Script native `SET_GROUP_CHAR_DUCKS_WHEN_AIMED_AT` (hash 0x5C8C7F9E).
///
/// Forwards a group handle and a bool flag to the engine. Quirk
/// (observed): the flag dword's high bytes repeat the context pointer (the
/// handler coerces into its own stack slot); the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00ba2050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
