// original: 0x00ba16c0 SET_CHAR_MAX_TIME_UNDERWATER
/// Script native `SET_CHAR_MAX_TIME_UNDERWATER` (hash 0x7110790B).
///
/// Forwards two script arguments to the engine: a character handle and a
/// float bit-pattern (the time limit). The float is copied as raw bits, so
/// the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba16c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
