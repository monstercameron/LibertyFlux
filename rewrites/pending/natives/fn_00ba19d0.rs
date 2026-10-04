// original: 0x00ba19d0 SET_CHAR_RELATIONSHIP
/// Script native `SET_CHAR_RELATIONSHIP` (hash 0x6D9538E1).
///
/// Forwards arg0, arg1, arg2 to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba19d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2));
        ans
    }
});
