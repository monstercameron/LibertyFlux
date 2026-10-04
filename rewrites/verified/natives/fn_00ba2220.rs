// original: 0x00ba2220 SET_NM_MESSAGE_BOOL
/// Script native `SET_NM_MESSAGE_BOOL` (hash 0x202F384E).
///
/// Forwards a message id and a bool value to the engine's natural-motion
/// messaging. Quirk (observed): the bool dword's high bytes repeat the
/// context pointer (the handler coerces into its own stack slot);
/// the rewrite passes the plain 0/1 flag and the contract masks
/// this call argument (`call_skip`); only the low byte is meaningful.
export!(cdecl, rw_00ba2220(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        callee_cdecl!(1, u32, *args, flag)
    }
});
